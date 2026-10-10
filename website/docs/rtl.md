---
title: "Arabic and RTL (Holool fork)"
description: "Inherited RTL layout, native Arabic editing, and GPUI Fast integration in the Holool fork."
---

# Arabic and RTL (Holool fork)

The Holool fork uses [Holool GPUI Fast](https://github.com/holool/gpui-fast) by
default. Its compatibility crates also keep Base, Component, and GPUI types on
one engine when a consumer disables Kit's default features. Use the Git source
and its checked-in lockfile; these changes are not published on crates.io.

```rust
use gpui_kit::{Directional as _, LayoutDirection, ParentElement as _, Styled as _, div, px};
let content = div()
    .direction(LayoutDirection::Rtl)
    .ps(px(12.))
    .child("مرحبا بالعالم")
    .child(div().ltr().child("LTR source code"));
```

Direction inherits through styled elements, retained views, and deferred draws.
Taffy handles RTL flex and grid positioning without changing source, focus, or
accessibility order. Use `ps`/`pe`, `ms`/`me`, and `start`/`end` for logical edges.
Physical `pl`/`pr` and `left`/`right` keep their original meanings. Set `.ltr()` on
code editors or paths when that is the desired visual policy. Direction defaults
text alignment to inline start; an explicit physical alignment takes precedence.

Inputs and textareas inherit text direction and alignment. Ordinary shaped rows
use bidi caret positions for hit testing, arrows, Shift+arrows, and disjoint
selection fills, including movement across hard breaks and soft wraps. Empty hints paint separately from the zero-length document, so
placeholder width and script do not move the caret or scroll an empty input.
IME candidate rectangles normalize reversed horizontal endpoints. Switches put
checked thumbs at inline end; breadcrumb and sidebar chevrons follow direction;
popup submenus prefer the reading side and retain their edge fallback. Tree
expansion arrows follow the tree's last rendered direction during event dispatch.

Run `cargo run -p arabic_rtl` to try Arabic marks, mixed English/numbers, an empty
hint, multiline editing, switches, a nested LTR island, and live direction changes.

## Port decisions

| Existing gpui-arabic change | Treatment here |
| --- | --- |
| Inherited layout direction | Implemented in Fast's style context and Taffy 0.14, including retained cache invalidation. |
| Text/caret geometry | Adapted to native shaping; CE's Parley implementation is not transplanted. |
| Final content-width reconciliation | Kit already shapes input text from final prepaint bounds. |
| Empty placeholders/carets | Ported to Kit's shared Input/textarea/editor engine. |
| Label/focus/accessibility wiring | Kit already owns these behaviors in Base and Component. |
| Switch drag semantics | Kit exposes a controlled click/keyboard switch; no unrelated drag API is introduced. |
| X11 touchscreen contacts | Ported to Fast's Linux backend with contact/cancellation tests. |
| Hardware-before-software GPU selection | Already present in Fast. |

This is explicit LTR/RTL layout, not CSS `direction:auto` inference. Mixed-run
boundary carets currently have a single primary position; a separate native caret
affinity API is not added. Inline token fragments and custom physical padding or
icons remain application presentation policy and need separate review before
claiming arbitrary RTL token layouts. See Fast's `docs/holool-rtl.md` for native
shaping details and platform QA limits.

The full Base suite has 17 failures that also reproduce with unmodified upstream Kit and Fast; see [validation details](https://github.com/holool/gpui-kit/blob/codex/arabic-rtl/docs/rtl-validation.md).

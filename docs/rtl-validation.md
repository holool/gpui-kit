# RTL validation

## Initial port (October 10, 2026)

Tested on macOS with Rust 1.95, plus Linux/X11 in a Rust 1.95 ARM64 container
and a cross-compiled x86_64 Windows GNU backend. Native Windows drawing and
physical touch devices still need device QA.

- GPUI Fast core: 640 passed, 3 ignored.
- Kit Base with RTL: 1,313 passed, 17 failed.
- Unmodified Kit + unmodified upstream Fast: 1,310 passed, the same 17 failed.
- Kit input-element regression group: 40 tests (including native Arabic,
  placeholders, decoration ranges, and newline navigation) pass as part of Base.
- Kit switches: 10 passed, including both states in both directions.
- Kit facade, UI integrations, and doctests: 382 passed. The Tig example builds.
- Documentation: full site build, 9 documentation tests, 15 metadata tests,
  and 3 internal-link tests pass.
- Engine CI passes on macOS, Linux, and Windows, including lint/upstream checks.
  Kit keeps the exact published Kit-aware macro snapshot; runtime crates share
  the pinned Fast commit. Procedural macros carry no runtime types.
- Native CoreText shaping/wrapping, X11 contact tracking, Windows cross-check,
  Arabic example build, GPUI clippy, and Fast upstream-hook checks are covered.
- Strict Kit clippy reports two existing `nonminimal_bool` expressions in
  `crates/base/src/calendar.rs:132`; it passes with that one lint allowed.

The baseline comparison used Kit `ed27c6293b585acc40c10be7138534243bee5841`
and Fast `b8f0c026553711c1e049fee501932d3801f23567`, in the same checkout paths
and feature configuration. Source changes were temporarily restored to their
upstream versions for the comparison; both forks' changes were then restored.
The baseline failures were:

```
dock::tab_group::tests::an_empty_group_asks_the_renderer_for_its_empty_state
dock::tab_group::tests::the_renderer_composes_frame_then_tab_bar_then_content
input::state::tests::test_input_does_not_invalidate_cached_parent_during_paint
motion::tests::a_completed_transition_stops_requesting_frames
motion::tests::presence_enters_exits_and_only_unmounts_after_exit
motion::tests::status_transition_reports_delay_running_and_finished
nav_stack::tests::the_outgoing_view_is_dropped_once_its_exit_has_run
popover::tests::default_open_renders_content_without_activation
popup::tests::the_popup_surface_blocks_the_panel_it_covers
popup::tests::trigger_capture_enables_deferred_content_on_the_next_frame
scrollbar::tests::repeated_touch_and_mouse_drags_keep_the_painted_grab_point
text::state::tests::reveal_range::a_reveal_that_cannot_be_shown_gives_up
text::text_view::tests::ordered_markdown_list_start_reaches_layout_marker
toast::tests::bottom_stack_reflow_moves_up_without_an_opposite_direction_jump
toast::tests::keyed_stack_reflow_moves_from_the_current_visual_position
virtual_list::tests::horizontal_visible_range_and_deferred_scroll_are_preserved
virtual_list::tests::vertical_visible_range_and_deferred_scroll_are_preserved
```

These failures are retained rather than skipped. The fork does not claim a green
full Kit suite or that this RTL port resolves all upstream Fast integration issues.

## Switch follow-up from gpui-arabic

Reviewed gpui-arabic commit `712cebb80b778bb4e96aee33b5193c1b09a48944`
(`Make BaseSwitch follow RTL direction`). CE's new fix resolves direction after
layout for its drag gesture, reverses the drag delta in RTL, and preserves the
resolved direction between gestures. Kit's existing Base switch supports click
and keyboard activation, with no drag state or drag threshold API, so this
follow-up does not copy CE's component or add a new gesture.

The styled Kit switch already resolves its explicit direction or inherits the
parent direction, and puts the checked thumb at inline end. Its Arabic example
already supports switching between RTL and LTR. No runtime change or Fast
engine update is needed for this source commit.

The Fast-backed switch regression now covers inherited direction, both explicit
overrides, changing the parent and control direction without changing identity,
removing an override, controlled click/Enter/Space activation, callback order,
focus, disabled controls, and thumb/label geometry in both states. Reduced
motion is enabled for deterministic endpoint assertions; this test does not
exercise animated travel or add drag support. Fast CI runs the entire switch
test group on macOS, Linux, and Windows. All 10 switch tests pass locally on
macOS with Rust 1.95 and `gpui-fast` enabled.

## Upstream Fast sync (October 16, 2026)

Kit pins all GPUI runtime crates to Holool Fast
`f36872ac33346aab252712e3bfd0e567807e6f4c`, a merge of Holool main and
longbridge/gpui-fast `eeccb27f7b7b437e0d99277af2b696e6c71fb6ef`.
The compatibility snapshot and the published Kit-aware procedural macros move
together to exact version 0.3.9. Fast remains enabled by default; dependency
inspection confirms one shared Fast runtime across the compatibility aliases.

CoreText moved into `gpui_apple`; Fast preserves the macOS native text-system
helper used by Kit's Arabic tests. The new upstream text-measurement cache guard
also recognizes independently shaped bidi soft wraps, retaining correct
unconstrained Arabic and mixed-text measurements.

The FPS test fixtures initialize the snapshot's new frame-timing fields.
Four bounce fixtures now refresh after their setup changes shared list offsets
outside entities. Upstream Fast classifies scrollbar offsets as scroll-only
changes; the old fixtures relied on the earlier broad invalidation. The refresh
is before the tested gestures, preserving their existing coalescing and boundary
assertions. No production Base behavior changed. Before this fixture adjustment,
the new engine had four failures and unchanged Holool main passed all 23 bounce
tests; afterward all 23 pass with the new engine.

Local validation with Rust 1.95:

- Fast core: 694 passed, 3 ignored on macOS; 708 passed, 3 ignored on Linux.
- Native CoreText Arabic regression: 1 passed; Linux/X11 contacts: 5 passed.
- Full Kit Base: 1,337 passed, including native Arabic editing regressions.
- Kit facade, UI integrations, and doctests: 382 passed.
- Kit switches: 10 passed; FPS: 34 passed.
- Kit offscreen rendering: 12 passed with the real Metal renderer.
- Explicit Fast and no-default-features checks pass; Arabic and Tig examples build.
- Exact snapshot-pin check, formatting, and upstream-hook checks pass.
- Strict core/Apple/macOS and profiler Clippy pass. Kit all-targets Clippy passes
  with only the previously documented calendar `nonminimal_bool` lint allowed.

The counts above supersede the initial-port failure counts for this revision.
Native Windows and physical touchscreen QA remain outside local validation;
platform CI runs on the companion PRs.

## Upstream Kit sync (October 16, 2026)

Merged all 50 upstream commits through longbridge/gpui-kit
`2963d6d880b9dffe59b48b0533bdc83c03201209` (Kit 0.7.3). This includes the
unified Shell, optional Syntect backend, native overscroll, focus/select/input
fixes, and disabled-control accessibility reporting. The Arabic example now
uses the workspace's 0.7.3 version. Holool's Arabic shaping, bidi selection,
native focus, IME geometry, RTL controls, Settings search, motion timing, and
Fast test-observation fixes remain in the merged sources.

Fast stays in Kit's default feature list. The shared runtime revision remains
Holool `f36872ac33346aab252712e3bfd0e567807e6f4c`; all compatibility runtime
crates and the published Kit-aware macros remain exactly 0.3.9. Cargo metadata
confirms exactly one GPUI runtime from that Holool revision, including the
compatibility aliases. The upstream development profiles now name the actual
`gpui` and `gpui_platform` packages.

Two new input-overscroll fixtures deliver their requested animation frames with
`simulate_next_frame` before drawing. Their original endpoint assertions and
120-frame limit remain intact. Three new tooltip checks counted builder calls
on idle draws; they now notify the tooltip overlay before checking its builder
ownership, allowing other frames to retain the existing view.

The full component suite also exposed stale dock close buttons when a shared
skin setting changed. Skin settings live in `Cell`s outside retained entities.
The skin now remembers its owning window and defers a refresh of that window
when a setting changes, including from an in-progress UI update. This preserves
normal-frame caching and does not refresh other windows. The existing
close-button visibility regression exercises both enable and disable transitions.

Local validation on macOS with Rust 1.95:

- Full Base: 1,389 passed, including native Arabic editing, selection, focus,
  and the new disabled-accessibility regressions.
- Component: 690 passed; component integration: 50 passed, including the
  dock setting transition and all RTL switches.
- Kit facade/UI integration: 391 passed; Kit doctests: 2 passed.
- Real Metal offscreen rendering: 13 passed.
- FPS: 34 passed; WebView: 2 passed; optional Syntect highlighter: 6 passed.
- Shell: 117 integration tests and 919 runtime tests passed; 1 upstream large
  benchmark ignored by default.
- Explicit Fast and no-default-features checks pass; Arabic, Tig, and Syntect
  examples compile.
- Generated TypeScript positive/negative fixtures and runtime checks pass.
  JS Story coverage tracks all 78 catalog surfaces across 77 routes.
- Exact snapshot pins, the shared Fast dependency graph, formatting, and
  published documentation fragment checks pass.
- All-targets strict core/Shell/WebView Clippy and Cargo Machete pass without
  lint allowances or unused dependencies.

Shell's temporary Git fixtures inherit this machine's signed-tag default, so
validation overrides `tag.gpgSign` and `commit.gpgSign` only in the test process.
No global Git configuration changes. Parallel Shell runs exposed an upstream
500 ms loopback-accept timeout and a wall-clock templating benchmark assertion;
the entire runtime suite is also run serially, with no added skips or weakened
assertions. Its one ignored large benchmark is upstream's existing default.

Native Windows, Linux GUI, OS IME, physical touch, and manual assistive-technology
review are outside this local macOS validation; the PR retains platform CI and
Holool's RTL CI steps.


## October 20, 2026 upstream sync

Merge longbridge/gpui-kit through `1676826d9d49b90f4d7b6808b69924f58796f6f1`
(Kit 0.8.0), preserving Holool's Arabic editing, native bidi geometry,
selection, controls, table stripe filler, RTL progress and mobile integration.
Every runtime alias and compatibility package uses Holool Fast
`4152f0914665ffe06be31820e3f48ca4be39227c`; exact compatibility/macros remain
0.3.9 and Fast remains in Kit's default features.

The native adapters are coordinated at iOS
`bbf4cbb394c18d79cb1bc9c3a84712d04a5d5c91` and Android
`e494c5c2743106ce01cc1b201de24614230dfecf`. Their examples use the same Fast
revision. The engine preserves the Android GLES and exclusive-surface fixes
while importing upstream's wgpu 30 renderer migration.

CE's empty-directional-source crash is addressed at the equivalent Fast
boundary: its existing source-empty guard runs before inserting a direction
marker. Native CoreText and isolated-font Cosmic regressions exercise empty
source with no font runs, zero-length runs, both directions, cached calls and
wrapping. The Kit regression mounts empty Input/Textarea fields, changes
inherited direction, adds/removes Arabic text and placeholders, and draws
multiple retained frames while checking source lengths and caret offsets.
No CE/Parley backend or legacy gpui-arabic dependency is introduced.

Resolve upstream's immutable copy-selection geometry with Holool's physical
RTL row extents. Preserve our exact-duration motion helper (upstream now has
the equivalent precision fix). Import the new tooltip trigger-removal behavior
and its retained-frame-compatible tests. Upstream's WebView feature now follows
Kit's backend explicitly; enable both Fast features together for composition.

Target-device rendering, real mobile keyboards, native Windows/Linux UI and
assistive-technology acceptance remain distinct from automated compile and
headless checks. Current validation results are recorded in the sync PR.

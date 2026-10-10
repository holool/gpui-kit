# RTL validation

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

//! Source byte ranges for immutable snapshots of shaped visual rows.
use gpui::{Pixels, Point, WrappedLineLayout};
use smallvec::SmallVec;
use std::{ops::Range, sync::Arc};

pub(super) fn line_ranges(
    text: &str,
    lines: &[Arc<WrappedLineLayout>],
) -> SmallVec<[Range<usize>; 1]> {
    let mut start = 0;
    lines
        .iter()
        .map(|line| {
            let end = start + line.len();
            let range = start..end;
            // Fast can shape an RTL soft wrap as its own line. Only source newlines
            // consume a byte between rows; visual wrapping consumes none.
            start = end + usize::from(text.as_bytes().get(end) == Some(&b'\n'));
            range
        })
        .collect()
}

pub(super) fn position_for_index(
    lines: &[Arc<WrappedLineLayout>],
    ranges: &[Range<usize>],
    mut origin: Point<Pixels>,
    line_height: Pixels,
    index: usize,
) -> Option<Point<Pixels>> {
    for (ix, (line, range)) in lines.iter().zip(ranges).enumerate() {
        if index < range.start {
            break;
        }
        // At a soft-wrap boundary the caret belongs to the next visual row.
        let continues = ranges
            .get(ix + 1)
            .is_some_and(|next| next.start == range.end);
        if index <= range.end && !(index == range.end && continues) {
            return Some(origin + line.position_for_index(index - range.start, line_height)?);
        }
        origin.y += line.size(line_height).height;
    }
    None
}

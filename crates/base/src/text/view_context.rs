//! Keep a TextView's ambient context inside its retained view boundary.
//! Fast can repaint that view without replaying the outer TextView element.
use super::TextViewState;
use crate::GlobalState;
use gpui::{
    App, Bounds, Element, ElementId, Entity, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Window,
};

pub(super) fn with_view_context<E: IntoElement>(
    content: E,
    state: Entity<TextViewState>,
) -> impl IntoElement {
    TextViewContext {
        element: content.into_element(),
        state,
    }
}

struct TextViewContext<E> {
    element: E,
    state: Entity<TextViewState>,
}

impl<E: Element> IntoElement for TextViewContext<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for TextViewContext<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;

    fn id(&self) -> Option<ElementId> {
        self.element.id()
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.element.source_location()
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.element.request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        GlobalState::global(cx).push_text_view_state(self.state.clone());
        let state = self
            .element
            .prepaint(id, inspector, bounds, layout, window, cx);
        GlobalState::global(cx).pop_text_view_state();
        state
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.state.read(cx).selectable {
            self.state.read(cx).selection_adapter.begin_frame();
        }
        GlobalState::global(cx).push_text_view_state(self.state.clone());
        self.element
            .paint(id, inspector, bounds, layout, prepaint, window, cx);
        GlobalState::global(cx).pop_text_view_state();
    }
}

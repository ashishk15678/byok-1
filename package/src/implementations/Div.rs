use gpui::AnyElement;
use gpui::Div;
use gpui::IntoElement;

impl IntoElement for Div {
    fn into_any_element(self) -> gpui::AnyElement {
        AnyElement { 0: self }
    }
}

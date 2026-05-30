use dioxus::prelude::*;

/// Wraps children in a div that fades in after initial render via CSS transitions.
///
/// The first render uses class `reveal-section` (opacity 0, translateY).
/// `use_effect` fires post-paint, setting `visible` to true, which adds
/// `reveal-visible` (opacity 1, translateY 0). The CSS `transition` on
/// `.reveal-section` handles the animation.
///
/// Children stagger with nth-child delays for a cascading reveal effect.
#[component]
pub fn ScrollReveal(children: Element) -> Element {
    let mut visible = use_signal(|| false);

    use_effect(move || {
        visible.set(true);
    });

    let class = if visible() {
        "reveal-section reveal-visible"
    } else {
        "reveal-section"
    };

    rsx! {
        div { class: "{class}", {children} }
    }
}

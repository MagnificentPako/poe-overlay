use html::Div;
use leptos::*;
use leptos_use::*;

#[component]
pub fn BaseWindow(title: TextProp, children: Children) -> impl IntoView {
    let bar_ref = create_node_ref::<Div>();
    let window_ref = create_node_ref::<Div>();
    let (size, set_size) = create_signal("".to_string());

    let UseDraggableReturn { style, .. } = use_draggable(bar_ref);

    use_resize_observer(window_ref, move |entries, observer| {
        let rect = entries[0].content_rect();
        set_size.set(format!(
            "width: {}px; height: {}px",
            rect.width(),
            rect.height()
        ));
    });

    view! {
        <div class="window"
            style=move || format!("{};{}", style.get(), size.get())
            _ref=window_ref
        >
            <div class="title-bar" _ref=bar_ref>
                <p>{title}</p>
            </div>
            <div class="window-body">
                {children()}
            </div>
        </div>
    }
}

use crate::DEFAULT_CSS_COLLECTION;
use dioxus::prelude::*;
use once_cell::sync::Lazy;

use std::collections::HashMap;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct StyleListeners(Arc<Mutex<StyleListenersInner>>);

impl StyleListeners {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(StyleListenersInner::new())))
    }
}

struct StyleListenersInner {
    listeners: HashMap<ScopeId, Arc<dyn Fn() + Send + Sync + 'static>>,
}

impl StyleListenersInner {
    fn new() -> Self {
        StyleListenersInner {
            listeners: HashMap::new(),
        }
    }
}

static STYLE_LISTENERS: Lazy<StyleListeners> = Lazy::new(|| StyleListeners::new());

struct StyleListener {
    listeners: StyleListeners,
    id: ScopeId,
}

impl StyleListener {
    fn register(
        listeners: StyleListeners,
        id: ScopeId,
        update: Arc<dyn Fn() + Send + Sync + 'static>,
    ) -> Self {
        {
            let mut lock = listeners.0.lock().expect("Could not acquire lock");
            lock.listeners.insert(id, update);
        }

        Self { listeners, id }
    }
}

impl Drop for StyleListener {
    fn drop(&mut self) {
        let mut lock = self.listeners.0.lock().expect("Could not acquire lock");
        lock.listeners.remove(&self.id);
    }
}

/// Registers a style listener – whenever style listeners will be notified, the component will be re-rendered.
fn use_style_listener(cx: &ScopeState) {
    let _listener = cx.use_hook(|| {
        StyleListener::register(
            (*STYLE_LISTENERS).clone(),
            cx.scope_id(),
            cx.schedule_update(),
        )
    });
}

#[doc(hidden)]
pub fn notify_dioxus_listeners() {
    let lock = STYLE_LISTENERS.0.lock().expect("Could not acquire lock");

    for update_fn in lock.listeners.values() {
        (update_fn)();
    }
}

/// A Dioxus component that renders the app's CSS in a `<style>` element.
///
/// (The CSS will come from the `css!` and `global_css!` macros throughout your app)
///
/// The element will be rendered in place. While it would technically be better to keep styles in the `<head>`, putting it elsewhere [is probably fine](https://softwareengineering.stackexchange.com/questions/224422/will-it-be-a-wrong-idea-to-have-style-in-body)
///
/// Example:
/// ```
/// use dioxus::prelude::*;
/// use sir::AppStyle;
///
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx!(AppStyle {}))
/// }
/// ```
#[allow(non_snake_case)]
pub fn AppStyle(cx: Scope) -> Element {
    let css = DEFAULT_CSS_COLLECTION.get_css();

    use_style_listener(&cx);

    // putting style element wherever is probably fine
    // https://softwareengineering.stackexchange.com/questions/224422/will-it-be-a-wrong-idea-to-have-style-in-body
    cx.render(rsx!(style {"{css}"}))
}

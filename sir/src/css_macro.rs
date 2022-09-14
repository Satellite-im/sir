/// Generates a CSS class and declares a SCSS rule for that class.
///
/// The SCSS will be converted to CSS at compile time. At runtime, when the `css!` line is executed, the CSS will be registered into the global CSS repository.
/// The generated class name will be returned, and you should add it to the component you wish to style.
///
/// If you don't want to use a generated class name, use the `global_css!` macro,
///
/// Example:
/// ```
/// use sir::css;
///
/// let button = css!("
///     background: deepskyblue;
///     transition: background 0.2s ease-out;
///
///     &:hover {
///         background: aquamarine;
///     }
/// ");
/// // now, render a component and specify the provided class name
///```
#[macro_export]
macro_rules! css {
    ($body:literal) => {{
        // Register globally once.
        static IS_REGISTERED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        let was_registered = IS_REGISTERED.swap(true, std::sync::atomic::Ordering::Relaxed);

        let (class, css) = sir::compile_style_item!($body);

        if !was_registered {
            sir::DEFAULT_CSS_COLLECTION.register(css.to_string());

            sir::notify_dioxus_listeners!();
        }

        class
    }};
}

/// Declares SCSS rules
///
/// The SCSS will be converted to CSS at compile time. At runtime, when the `global_css!` line is executed, the CSS will be registered into the global CSS repository.
///
/// Unlike the `css!` macro, this style will not be specific to a single class – you must provide your own selectors.
///
/// Example:
/// ```
/// use sir::global_css;
///
/// global_css!("
///     body {
///         background: slategray;
///     }
/// ");
///```
#[macro_export]
macro_rules! global_css {
    ($body:literal) => {{
        // Register globally once.
        static IS_REGISTERED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        let was_registered = IS_REGISTERED.swap(true, std::sync::atomic::Ordering::Relaxed);

        let css = sir::compile_global_style!($body);

        if !was_registered {
            sir::DEFAULT_CSS_COLLECTION.register(css.to_string());

            sir::notify_dioxus_listeners!();
        }
    }};
}

#[doc(hidden)]
#[cfg(not(feature = "dioxus"))]
#[macro_export]
macro_rules! notify_dioxus_listeners {
    () => {};
}

#[doc(hidden)]
#[cfg(feature = "dioxus")]
#[macro_export]
macro_rules! notify_dioxus_listeners {
    () => {
        sir::notify_dioxus_listeners();
    };
}

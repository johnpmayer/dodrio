use dodrio::{
    builder::*,
    bumpalo::{self, Bump},
};
use dodrio::{Node, Render, Vdom};
use wasm_bindgen::prelude::*;

/// A rendering component that displays a greeting.
struct Hello {
    /// Who to greet.
    who: String,
}

// The `Render` implementation describes how to render a `Hello` component into
// HTML.
impl Render for Hello {
    fn render<'bump>(&self, bump: &'bump Bump) -> Node<'bump> {
        let msg = bumpalo::format!(in bump, "Hello, {}!", self.who);
        let msg = msg.into_bump_str();
        p(bump).children([text(msg)]).finish()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    // Set up the panic hook for debugging when things go wrong.
    console_error_panic_hook::set_once();

    // Grab the document's `<body>`.
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();

    // Create a new `Hello` render component.
    let component = Hello {
        who: String::from("World"),
    };

    // Create a virtual DOM and mount it and the `Hello` render component to the
    // `<body>`.
    let vdom = Vdom::new(body.as_ref(), component);

    // Run the virtual DOM forever and don't unmount it.
    vdom.forget();
}

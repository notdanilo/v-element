use web_component::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use js_sys::Function;

#[derive(WebComponent, Serialize, Deserialize, Clone)]
pub struct Status {
    status: bool
}

impl WebComponent for Status {
    fn create_component(_attributes: NamedNodeMap) -> Self {
        Self { status: true }
    }

    fn on_loaded(&mut self, shadow_root: ShadowRoot) {
        let button = shadow_root
            .get_element_by_id("button")
            .expect("Couldn't find id=\"button\"")
            .dyn_into::<HtmlElement>()
            .expect("Couldn't convert to HtmlElement.");

        let mut clone = self.clone();
        let on_click = move || {
            web_sys::console::log_1(&format!("{}", clone.status).into());
            clone.update_field("status", &(!clone.status).to_string());
            web_sys::console::log_1(&format!("{}", clone.status).into());
        };
        let closure = Closure::wrap(Box::new(on_click) as Box<dyn FnMut()>);
        let function : &Function = closure.as_ref().unchecked_ref();

        button.set_onclick(Some(function));
        std::mem::forget(closure);
    }
}

template!(Status);
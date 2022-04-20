mod metadata;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use async_trait::async_trait;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, HtmlTemplateElement, ShadowRootInit, ShadowRootMode, window};
use crate::CustomElement;
use metadata::Metadata;

#[async_trait(?Send)]
pub trait WebComponent {
    fn new(element: HtmlElement) -> Self;

    fn element(&self) -> &HtmlElement;

    fn name(&self) -> String {
        js_sys::Reflect::get(&self.element(), &"name".into()).unwrap().as_string().unwrap()
    }

    fn path(&self) -> String {
        js_sys::Reflect::get(&self.element(), &"path".into()).unwrap().as_string().unwrap()
    }

    /// Draw interval in seconds. None will recursively call requestAnimationFrame.
    fn draw_interval(&self) -> Option<f64> { None }

    /// Run once, return true if you want to request a new animation frame.
    fn draw(&self) -> bool { false }

    async fn metadata(&self) -> Option<Metadata> { Default::default() }

    async fn connected(&mut self) {}

    async fn disconnected(&mut self) {}

    async fn adopted(&mut self) {}

    async fn attribute_changed(&mut self, _attribute_name: String, _old: Option<String>, _new: Option<String>) {}

    fn observed_attributes() -> Vec<String> where Self: Sized {
        vec![]
    }

    async fn template(&self) -> Option<String> {
        let path = self.path();
        let name = self.name();
        let index_path = format!("{}/{}.html", path, name);
        let window = window().unwrap();
        let response = JsFuture::from(window.fetch_with_str(&index_path)).await.unwrap();
        let response = response.dyn_into::<web_sys::Response>().unwrap();
        let text = response.text().unwrap();
        let text = JsFuture::from(text).await.unwrap().as_string().unwrap();
        Some(text)
    }
}

#[async_trait(?Send)]
impl<T: WebComponent> CustomElement for T {
    fn new(element: HtmlElement) -> Self {
        WebComponent::new(element)
    }

    fn element(&'static self) -> &HtmlElement {
        WebComponent::element(self)
    }

    async fn connected(&'static mut self) {
        self.element().attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open)).ok();
        let window = window().unwrap();
        if let Some(shadow_root) = self.element().shadow_root() {
            let document = window.document().unwrap();
            let template = document.create_element("div").unwrap();
            if let Some(text) = self.template().await {
                template.set_inner_html(&text);
            }
            let template = template.first_child().unwrap().dyn_into::<HtmlTemplateElement>().unwrap();
            let node = document.import_node_with_deep(&template.content(), true).unwrap();
            shadow_root.append_child(&node).ok();
        }
        T::connected(self).await;

        // Setting up draw loop
        let callback: Rc<RefCell<Option<Closure<_>>>> = Rc::new(RefCell::new(None));
        let callback_clone = callback.clone();
        *callback_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if let Some(callback) = callback.borrow().as_ref() {
                let callback = callback.as_ref().unchecked_ref();
                if T::draw(self) {
                    if let Some(interval) = T::draw_interval(self) {
                        // TODO: Use set_interval outside instead.
                        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(callback, (interval * 1000.0) as i32).unwrap();
                    } else {
                        web_sys::window().unwrap().request_animation_frame(callback).unwrap();
                    }
                }
            }
        }) as Box<dyn FnMut()>));
        let callback = callback_clone.borrow();
        let callback = callback.as_ref().unwrap();
        let callback = callback.as_ref().unchecked_ref();
        window.request_animation_frame(callback).unwrap();
    }

    async fn disconnected(&'static mut self) {
        T::disconnected(self).await
    }

    async fn adopted(&'static mut self) {
        T::adopted(self).await
    }

    async fn attribute_changed(&'static mut self, attribute_name: String, old_value: Option<String>, new_value: Option<String>) {
        T::attribute_changed(self, attribute_name, old_value, new_value).await
    }

    fn observed_attributes() -> Vec<String> {
        T::observed_attributes()
    }
}

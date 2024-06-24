use async_trait::async_trait;
use web_sys::HtmlElement;

#[async_trait(?Send)]
pub trait CustomElement {
    fn new(element: HtmlElement) -> Self where Self: Sized;

    fn element(&'static self) -> &HtmlElement;

    async fn connected(&'static mut self) {}

    async fn disconnected(&'static mut self) {}

    async fn adopted(&'static mut self) {}

    async fn attribute_changed(&'static mut self, _attribute_name: String, _old: Option<String>, _new: Option<String>) {}

    fn observed_attributes() -> Vec<String> where Self: Sized {
        vec![]
    }
}

pub type Identity = usize;

pub mod web_component;
pub use web_component::WebComponent;
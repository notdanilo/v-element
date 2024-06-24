use async_trait::async_trait;
use web_sys::{HtmlElement};
use web_component::WebComponent;

impl Drop for DigitalClock {
    fn drop(&mut self) {
        web_sys::console::log_1(&"DigitalClock dropped".into());
    }
}

pub struct DigitalClock {
    pub element: HtmlElement
}

#[async_trait(?Send)]
impl WebComponent for DigitalClock {
    fn new(element: HtmlElement) -> Self {
        Self {element}
    }

    fn element(&self) -> &HtmlElement {
        &self.element
    }

    fn draw_interval(&self) -> Option<f64> { Some(1.0) }

    fn draw(&self) -> bool {
        let now = &String::from(js_sys::Date::new_0().to_time_string())[0..8];
        self.element().shadow_root().unwrap().set_inner_html(now);
        true
    }
}

web_component::define!(DigitalClock, digital_clock);
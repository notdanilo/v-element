use async_trait::async_trait;
use wasm_bindgen::JsCast;

use web_sys::{HtmlElement, CanvasRenderingContext2d, HtmlCanvasElement};
use web_component::WebComponent;

pub struct AnalogClock {
    pub element: HtmlElement,
    pub context: Option<CanvasRenderingContext2d>
}

#[async_trait(?Send)]
impl WebComponent for AnalogClock {
    fn new(element: HtmlElement) -> Self {
        let context = None;
        Self {element, context}
    }

    fn element(&self) -> &HtmlElement {
        &self.element
    }

    fn draw_interval(&self) -> Option<f64> {
        Some(1.0)
    }

    fn draw(&self) -> bool {
        if let Some(context) = &self.context {
            let now = js_sys::Date::new_0();
            let seconds = now.get_seconds();
            let minutes = now.get_minutes();
            let hours = now.get_hours();
            context.save();
            context.clear_rect(0.0, 0.0, 256.0, 256.0);
            Self::draw_background(context);
            Self::draw_pointer(context, seconds as f64 / 60.0, 0.8, 1.0);
            Self::draw_pointer(context, minutes as f64 / 60.0, 0.65, 2.0);
            Self::draw_pointer(context, hours as f64 / 12.0, 0.5, 4.0);
            context.restore();
        }
        true
    }

    async fn connected(&mut self) {
        let canvas = self
            .element
            .shadow_root()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        self.context = Some(canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap());
        self.context.as_ref().unwrap().set_line_cap("round");
    }
}

impl AnalogClock {
    fn draw_background(context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(128.0, 128.0, 128.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.stroke();
        for i in 0..12 {
            let angle = i as f64 / 12.0 * std::f64::consts::PI * 2.0;
            context.begin_path();
            let x = 128.0 + js_sys::Math::cos(angle) * 128.0 * 0.95;
            let y = 128.0 + js_sys::Math::sin(angle) * 128.0 * 0.95;
            context.move_to(x, y);
            let x = 128.0 + js_sys::Math::cos(angle) * 128.0 * 0.9;
            let y = 128.0 + js_sys::Math::sin(angle) * 128.0 * 0.9;
            context.line_to(x, y);
            context.stroke();
        }
    }

    fn draw_pointer(context: &CanvasRenderingContext2d, normalized_time: f64, length: f64, width: f64) {
        let angle = normalized_time * std::f64::consts::PI * 2.0 - std::f64::consts::PI / 2.0;
        context.set_line_width(width);
        context.begin_path();
        context.move_to(128.0, 128.0);
        let x = 128.0 + js_sys::Math::cos(angle) * 128.0 * length;
        let y = 128.0 + js_sys::Math::sin(angle) * 128.0 * length;
        context.line_to(x, y);
        context.stroke();
    }
}

web_component::define!(AnalogClock, analog_clock);
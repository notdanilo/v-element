#[macro_export]
macro_rules! define {
    ($component:ident, $name:ident) => {
        mod wasm_interface {
            use $crate::prelude::*;
            use $crate::{CustomElement, Identity};
            use $crate::registry::REGISTRY;
            use super::$component;

            self::concat_idents!(fn_name = $name, _constructor {
                #[wasm_bindgen]
                pub fn fn_name(element: HtmlElement) -> Identity {
                    unsafe {
                        REGISTRY.register_object(Box::new($component::new(element)))
                    }
                }
            });

            self::concat_idents!(fn_name = $name, _observed_attributes {
                #[wasm_bindgen]
                pub fn fn_name() -> Vec<JsValue> {
                    $component::observed_attributes()
                        .into_iter()
                        .map(|value| value.into())
                        .collect()
                }
            });
        }
    }
}
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, console};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document.get_element_by_id("container").unwrap();
        let _ = invoke("start", JsValue::NULL).await;

        for i in 1..10001 {
            add_div(&document, &container, i);
        }

        let elapsed = invoke("stop", JsValue::NULL).await.as_string().unwrap();
        add_result(&document, &container, elapsed);
    });

}

fn add_div(document: &Document, container: &Element, num: usize) {
    let item = document.create_element("div").unwrap();
    item.set_attribute("class", "item").unwrap();
    item.set_text_content(Some(&format!("{}", num)));
    container.append_child(&item).unwrap();
}

fn add_result(document: &Document, container: &Element, time: String) {
    let header = document.create_element("h1").unwrap();
    header.set_text_content(Some(&format!("The result in nanoseconds is: {}", time)));
    container.insert_adjacent_element("afterBegin", &header).unwrap();
}
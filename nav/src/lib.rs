use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, NodeList, window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    let nav: Element = document
        .query_selector("nav")
        .unwrap()
        .unwrap()
        .dyn_into()?;

    // Grab all <a> tags within <nav> and all sections by class
    let nav_links: NodeList = nav.query_selector_all("a")?.dyn_into()?;
    let sections: NodeList = document
        .query_selector_all(".content-section")?
        .dyn_into()?;

    for i in 0..nav_links.length() {
        // Retrieve and clone each link element to avoid moving it into the closure
        let link_el: Element = nav_links.item(i).unwrap().dyn_into()?;
        let link_clone = link_el.clone();

        // Clone NodeLists and document for closure capture, preventing FnOnce
        let nav_links_clone = nav_links.clone();
        let sections_clone = sections.clone();
        let document_clone = document.clone();

        // Wrap Rust closure so it can be used as a JS event listener (must be FnMut)
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();

            // Iterate cloned nav links to remove "active" class from all
            for j in 0..nav_links_clone.length() {
                let l = nav_links_clone
                    .item(j)
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap();
                l.class_list().remove_1("active").unwrap();
            }

            // Add "active" to the clicked link (clone) without moving original
            link_clone.class_list().add_1("active").unwrap();

            // Hide all sections by removing their "active" class
            for k in 0..sections_clone.length() {
                let s = sections_clone
                    .item(k)
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap();
                s.class_list().remove_1("active").unwrap();
            }

            // Determine target section via data attribute and activate it
            let target_id = link_clone.get_attribute("data-section").unwrap();
            if let Some(sec) = document_clone.get_element_by_id(&target_id) {
                sec.class_list().add_1("active").unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        // Attach the closure as a click listener and leak it to keep it alive
        link_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Prevent closure from being dropped
    }

    // Programmatically click the first nav link to set initial state
    if let Some(first) = nav_links.item(0) {
        first.dyn_into::<web_sys::HtmlElement>().unwrap().click();
    }

    Ok(())
}

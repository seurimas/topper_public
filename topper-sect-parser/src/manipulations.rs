use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    window, CssStyleSheet, Element, HtmlElement, HtmlIFrameElement, HtmlPreElement, Node,
};

pub fn push_styles_into_frame(frame: &HtmlIFrameElement) {
    let outer_style_sheet: JsValue = window()
        .unwrap()
        .document()
        .unwrap()
        .style_sheets()
        .get(0)
        .unwrap()
        .into();
    let inner_style_sheet: JsValue = frame
        .content_document()
        .unwrap()
        .style_sheets()
        .get(0)
        .unwrap()
        .into();
    let outer_style_sheet: CssStyleSheet = outer_style_sheet.into();
    let rules = outer_style_sheet.css_rules().unwrap();
    let inner_style_sheet: CssStyleSheet = inner_style_sheet.into();
    for idx in 0..rules.length() {
        let rule = rules.get(idx).unwrap();
        inner_style_sheet.insert_rule(rule.css_text().as_ref());
    }
}

pub fn move_scroll_indicator(frame: &HtmlIFrameElement, scroll_top: i32) {
    if let Ok(Some(scroll_indicator)) = frame
        .content_document()
        .unwrap()
        .query_selector(".scroll_indicator")
    {
        scroll_indicator.set_attribute("style", format!("top: {}px", scroll_top + 16).as_ref());
    }
}

pub fn rearrange_lines(frame: &HtmlIFrameElement) -> Vec<Element> {
    let document = frame.content_document().unwrap();
    let body = document.body().unwrap();
    let pre_block: HtmlPreElement = body.child_nodes().get(1).unwrap().dyn_into().unwrap();
    let mut lines: Vec<Vec<Node>> = vec![];
    let mut line: Vec<Node> = vec![];
    for node_idx in 0..pre_block.child_element_count() {
        let node = pre_block.child_nodes().get(node_idx).unwrap();
        if let Some(text) = node.text_content() {
            line.push(node);
            if text.ends_with("\n") {
                let new_line = line;
                line = vec![];
                lines.push(new_line);
            }
        }
    }
    let mut new_lines = vec![];
    for line in lines.iter() {
        let new_line = document.create_element("span").unwrap();
        new_line.set_attribute("class", "line").unwrap();
        for node in line.iter() {
            new_line.append_child(node).unwrap();
        }
        body.append_child(&new_line).unwrap();
        new_lines.push(new_line);
    }
    let scroll_indicator = document.create_element("div").unwrap();
    scroll_indicator
        .set_attribute("class", "scroll_indicator")
        .unwrap();
    body.append_child(&scroll_indicator).unwrap();
    new_lines
}

pub fn get_scroll_points(lines: &Vec<Element>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| line.clone().dyn_into::<HtmlElement>().unwrap().offset_top())
        .collect()
}

pub fn find_scroll_point(scroll_points: &Vec<i32>, scroll_top: i32) -> usize {
    let point = scroll_points.binary_search(&scroll_top);
    match point {
        Ok(line_idx) | Err(line_idx) => line_idx,
    }
}

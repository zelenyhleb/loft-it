/*******************************************************************************
 * Copyright (c) ArSysOp 2018-2022
 * 
 * RGM Sources are publicly available only for 
 * informational, review, analysis and consulting purposes.
 * 
 * Definitions, terms and conditions for using RGM Sources are stated by ArSysOp Source License version 1.0.
 * See http://arsysop.ru/licenses/rgm/ArSysOpSourceLicense-1.0.txt
 * 
 * RGM Sources are provided on "as is" basis. 
 * ArSysOp is not responsible for any damages, losses, legal prosecution 
 * or other consequences of any sort that using RGM Sources can cause to you 
 * (as an individual or Legal Entity), even if aware of such consequences.
 * 
*******************************************************************************/
use html_parser::{Dom, Element, Node};

use crate::test::Section;

pub fn parse(dom: Dom) -> Option<Vec<Section>> {
    let body = dom
        .children
        .get(0)
        .and_then(|node| node.element())
        .and_then(|element| element.children.get(1))
        .and_then(|node| node.element());
    match body {
        Some(element) => Some(toc(element)),
        None => None,
    }
}

fn only_elements(node: &Node) -> Option<&Element> {
    match node {
        Node::Element(element) => Some(element),
        _ => None,
    }
}

fn only_divs(element: &Element) -> bool {
    String::from("div").eq(&element.name)
}

fn toc(body: &Element) -> Vec<Section> {
    let children = &body.children;
    children
        .into_iter()
        .filter_map(&only_elements)
        .map(|wrapper| &wrapper.children)
        .flat_map(|vec| vec.into_iter())
        .filter_map(&only_elements)
        .filter(|element| element.name == "div")
        .map(|element| chapter(element))
        .collect::<Vec<Section>>()
}

fn chapter(element: &Element) -> Section {
    let children = &element.children;
    let content = children
        .into_iter()
        .filter_map(&only_elements)
        .filter(|element| {
            let attributes = &element.classes;
            String::from("div").eq(&element.name)
                && attributes
                    .into_iter()
                    .filter(|class| String::from("tocChapter").eq(*class))
                    .count()
                    > 0
        })
        .map(|chapter| &chapter.children)
        .flat_map(|vec| vec.into_iter())
        .filter_map(&only_elements)
        .filter(|element| only_divs(element))
        .map(|element| section(element))
        .collect::<Vec<Section>>();
    let id = match &element.id {
        Some(id) => id.clone(),
        None => "".to_owned(),
    };
    Section { id, content }
}

fn section(element: &Element) -> Section {
    let children = &element.children;
    let content = children
        .into_iter()
        .filter_map(&only_elements)
        .filter(|element| only_divs(element))
        .map(|element| section(element))
        .collect::<Vec<Section>>();
    let id = match &element.id {
        Some(id) => id.clone(),
        None => "".to_owned(),
    };
    Section { id, content }
}

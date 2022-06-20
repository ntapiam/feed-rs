use crate::model::{Category, Entry, Link};
use crate::parser::ParseFeedResult;
use crate::xml::{Element, NS};
use std::io::BufRead;

pub(crate) fn handle_arxiv_element<R: BufRead>(element: Element<R>, entry: &mut Entry) -> ParseFeedResult<()> {
    match element.ns_and_tag() {
        (NS::Arxiv, "primary_category") => entry.primary_category = handle_primary_category(element),
        (NS::Arxiv, "comment") => entry.comment = element.child_as_text(),
        (NS::Arxiv, "affiliation") => todo!(),
        (NS::Arxiv, "journal_ref") => entry.journal_ref = element.child_as_text(),
        (NS::Arxiv, "doi") => entry.doi = handle_doi(element),
        _ => {}
    }

    Ok(())
}

fn handle_primary_category<R: BufRead>(element: Element<R>) -> Option<Category> {
    // Always need a term
    if let Some(term) = element.attr_value("term") {
        let mut category = Category::new(&term);

        for attr in element.attributes {
            match attr.name.as_str() {
                "scheme" => category.scheme = Some(attr.value.clone()),
                "label" => category.label = Some(attr.value.clone()),

                // Nothing required for unknown attributes
                _ => {}
            }
        }

        Some(category)
    } else {
        // A missing category isn't fatal
        None
    }
}

fn handle_doi<R: BufRead>(element: Element<R>) -> Option<Link> {
    element.child_as_text().and_then(|text| {
        Some(Link {
            href: format!("https://dx.doi.org/{}", text).into(),
            rel: Some("related".into()),
            media_type: Some("text/html".into()),
            href_lang: Some("en".into()),
            title: Some(text.into()),
            length: None,
        })
    })
}

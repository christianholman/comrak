extern crate comrak;

fn small() {
    use comrak::{markdown_to_html, ComrakOptions};

    assert_eq!(markdown_to_html("Hello, **世界**!", &ComrakOptions::default()),
               "<p>Hello, <strong>世界</strong>!</p>\n");
}

fn large() {
    use comrak::{parse_document, format_html, Arena, ComrakOptions};
    use comrak::nodes::{AstNode, NodeValue};

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
        &ComrakOptions::default());

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
        where F : Fn(&'a AstNode<'a>) {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| {
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
            }
            _ => (),
        }
    });

    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    assert_eq!(
        String::from_utf8(html).unwrap(),
        "<p>This is your input.</p>\n\
         <ol>\n\
         <li>Also your input.</li>\n\
         <li>Certainly your input.</li>\n\
         </ol>\n");
}

fn main() {
    small();
    large();
}

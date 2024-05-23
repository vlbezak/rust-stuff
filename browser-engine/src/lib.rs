use dom::pretty_print;

mod dom;
mod html_parsers;
mod css;
mod css_parsers;

pub fn add_node() {
    let node = dom::create_node();
    pretty_print(&node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_too() {
        assert!(true);
    }

    #[test]
    fn it_works() {
        add_node();
    }
}

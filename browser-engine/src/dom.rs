use std::collections::{HashMap, HashSet};
use std::fmt;

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String,String>;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children
        }
    }
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(c) => c.split(' ').collect::<HashSet<_>>(),
            None => HashSet::new(),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes_string = String::new();
        for (attr_name, attr_value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
        }
        write!(f, "{},{}", self.tag_name, attributes_string)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Comment(ref t) | NodeType::Text(ref t) => write!(f, "{}", t),
            NodeType::Element(ref t) => write!(f, "{:?}", t),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //f.debug_struct("Node").field("children", &self.children).field("node_type", &self.node_type).finish()
        write!(f, "{:?}", self.node_type)
    }
}

pub fn create_node() -> Node {

    println!("Starting");

    let node_type1 = NodeType::Text("text_node1".to_string());
    let node1 = Node::new(node_type1, vec![]);

    let node_type2 = NodeType::Text("text_node2".to_string());
    let node2 = Node::new(node_type2, vec![]);

    let mut root_element_data_attrs = AttrMap::new();
    root_element_data_attrs.insert("attr1".to_string(), "value1".to_string());
    root_element_data_attrs.insert("attr2".to_string(), "value2".to_string());

    let root_element_data = ElementData::new("root".to_string(), root_element_data_attrs);

    let root_type = NodeType::Element(root_element_data);
    let root = Node::new(root_type, vec![node1, node2]);
    root
}

pub fn pretty_print(n: &Node) {
    pretty_print_with_indent(n, 0);
}

fn pretty_print_with_indent(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_|" ").collect::<String>();
    
    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
        NodeType::Text(ref t) => println!("{}{}", indent, t),

    }
    
    for child in n.children.iter() {
        pretty_print_with_indent(child, indent_size + 2);
    }
}
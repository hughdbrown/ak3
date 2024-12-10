use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Node};

#[derive(Debug, Clone)]
enum VNode {
    Element {
        tag: String,
        props: HashMap<String, String>,
        children: Vec<VNode>,
    },
    Text(String),
}

struct VDom {
    document: Document,
    root: Element,
    current_tree: Option<VNode>,
}

impl VDom {
    fn new(root_id: &str) -> Result<Self, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root = document.get_element_by_id(root_id).unwrap();

        Ok(VDom {
            document,
            root,
            current_tree: None,
        })
    }

    fn create_element(&self, vnode: &VNode) -> Result<Node, JsValue> {
        match vnode {
            VNode::Element { tag, props, children } => {
                let element = self.document.create_element(tag)?;
                
                for (key, value) in props {
                    element.set_attribute(key, value)?;
                }
                
                for child in children {
                    let child_node = self.create_element(child)?;
                    element.append_child(&child_node)?;
                }
                
                Ok(element.into())
            }
            VNode::Text(text) => {
                Ok(self.document.create_text_node(text).into())
            }
        }
    }

    fn update(&mut self, new_tree: VNode) -> Result<(), JsValue> {
        match &self.current_tree {
            Some(old_tree) => {
                let patches = self.diff(old_tree, &new_tree);
                self.apply_patches(&patches)?;
            }
            None => {
                let node = self.create_element(&new_tree)?;
                self.root.append_child(&node)?;
            }
        }
        
        self.current_tree = Some(new_tree);
        Ok(())
    }

    fn diff(&self, old: &VNode, new: &VNode) -> Vec<Patch> {
        let mut patches = Vec::new();
        self.diff_nodes(old, new, &mut patches);
        patches
    }

    fn diff_nodes(&self, old: &VNode, new: &VNode, patches: &mut Vec<Patch>) {
        match (old, new) {
            (VNode::Element { tag: old_tag, props: old_props, children: old_children },
             VNode::Element { tag: new_tag, props: new_props, children: new_children }) => {
                if old_tag != new_tag {
                    patches.push(Patch::Replace(new.clone()));
                    return;
                }

                // Diff properties
                for (key, new_value) in new_props {
                    match old_props.get(key) {
                        Some(old_value) if old_value != new_value => {
                            patches.push(Patch::SetAttribute(
                                key.clone(),
                                new_value.clone(),
                            ));
                        }
                        None => {
                            patches.push(Patch::SetAttribute(
                                key.clone(),
                                new_value.clone(),
                            ));
                        }
                        _ => {}
                    }
                }

                // Diff children
                let min_len = old_children.len().min(new_children.len());
                for i in 0..min_len {
                    self.diff_nodes(&old_children[i], &new_children[i], patches);
                }

                if new_children.len() > old_children.len() {
                    patches.extend(new_children[min_len..].iter().map(|node| {
                        Patch::AppendChild(node.clone())
                    }));
                } else if old_children.len() > new_children.len() {
                    patches.push(Patch::RemoveChildren(
                        old_children.len() - new_children.len()
                    ));
                }
            }
            (VNode::Text(old_text), VNode::Text(new_text)) => {
                if old_text != new_text {
                    patches.push(Patch::SetText(new_text.clone()));
                }
            }
            _ => {
                patches.push(Patch::Replace(new.clone()));
            }
        }
    }

    fn apply_patches(&self, patches: &[Patch]) -> Result<(), JsValue> {
        for patch in patches {
            match patch {
                Patch::Replace(node) => {
                    let new_elem = self.create_element(node)?;
                    // Apply replacement logic
                }
                Patch::SetAttribute(key, value) => {
                    // Apply attribute update
                }
                Patch::SetText(text) => {
                    // Update text content
                }
                Patch::AppendChild(node) => {
                    let new_child = self.create_element(node)?;
                    // Append new child
                }
                Patch::RemoveChildren(count) => {
                    // Remove specified number of children
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Patch {
    Replace(VNode),
    SetAttribute(String, String),
    SetText(String),
    AppendChild(VNode),
    RemoveChildren(usize),
}

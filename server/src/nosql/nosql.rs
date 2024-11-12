use std::fmt::Display;

use super::value::Value;

pub struct Node {
    k: String,
    v: Value
}

impl Node {
    pub fn new(k: String, v: Value) -> Node {
        Node { k, v }
    }
}

pub struct RuCache {
    data: Vec<Node>
}

impl RuCache {
    pub fn new() -> RuCache {
        RuCache { data: Vec::new() }
    }

    pub fn len(&self) -> usize { self.data.len() }

    pub fn put(&mut self, k: String, v: Value) -> Result<bool, bool> {
        let new_node = Node::new(k, v);
        
        let pos = self.data.binary_search_by(|n| n.k.cmp(&new_node.k));

        match pos {
            Ok(i) => {
                self.data[i].v = new_node.v;
                return Ok(true);
            },
            Err(i) => {
                self.data.insert(i, new_node);
                return Ok(false);
            },
        }
    }

    pub fn delete(&mut self, k: &str) -> Result<bool, bool> {
        let pos = self.data.binary_search_by(|n| n.k.as_str().cmp(k));

        if let Ok(i) = pos {
            self.data.remove(i);
            return Ok(true);
        } else {
            return Err(false);
        }
    }

    pub fn get(&mut self, k: &str) -> Option<&Value> {
        if let Ok(index) = self.data.binary_search_by(|node| node.k.as_str().cmp(k)) {
            Some(&self.data[index].v)
        } else {
            None
        }
    }
}

impl Display for RuCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.data.iter() {
            writeln!(f, "Key: {}, Value: {}", n.k, n.v)?;
        }
        Ok(())
    }
}

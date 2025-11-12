use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::iter::zip;

#[derive(Debug)]
pub struct Node {
    left: Option<u32>,
    center: Option<u32>,
    right: Option<u32>,
}

impl Node {
    pub fn try_insert(&mut self, n: u32) -> bool {
        if self.center.is_none() {
            self.center = Some(n);
            true
        } else if n < self.center.unwrap() && self.left.is_none() {
            self.left = Some(n);
            true
        } else if n > self.center.unwrap() && self.right.is_none() {
            self.right = Some(n);
            true
        } else {
            false
        }
    }

    pub fn new() -> Node {
        Node {
            left: None,
            center: None,
            right: None,
        }
    }

    pub fn to_int(&self) -> u32 {
        let mut out_str: String = "".to_string();
        if let Some(n) = self.left {
            out_str += &(n.to_string())[..];
        }
        if let Some(n) = self.center {
            out_str += &(n.to_string())[..];
        }
        if let Some(n) = self.right {
            out_str += &(n.to_string())[..];
        }

        out_str.parse::<u32>().unwrap()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.to_int() == other.to_int()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_int().partial_cmp(&other.to_int())
    }
}

#[derive(Debug)]
pub struct Fishbone(Vec<Node>);

impl Fishbone {
    pub fn new() -> Fishbone {
        Fishbone(Vec::<Node>::new())
    }
    pub fn build_fishbone(nums: Vec<u32>) -> Fishbone {
        let mut f = Fishbone::new();
        if f.len() == 0 {
            f.0.push(Node::new())
        }

        for n in nums {
            let mut status: bool = false;
            for node in f.0.iter_mut() {
                status = node.try_insert(n);
                if status {
                    break;
                }
            }
            if !status {
                f.0.push(Node {
                    left: None,
                    center: Some(n),
                    right: None,
                });
            }
        }

        f
    }
    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_quality(&self) -> String {
        self.0.iter().fold("".to_string(), |acc, n| {
            acc + &(n.center.unwrap().to_string())[..]
        })
    }

    pub fn get_quality_int(&self) -> u64 {
        self.get_quality().parse::<u64>().unwrap()
    }
}

impl PartialEq for Fishbone {
    fn eq(&self, other: &Self) -> bool {
        if self.get_quality_int() != other.get_quality_int() {
            return false;
        }
        for (a, b) in zip(&self.0, &other.0) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Fishbone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_quality_int().partial_cmp(&other.get_quality_int()) {
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Equal) => {
                for (a, b) in zip(&self.0, &other.0) {
                    if a < b {
                        return Some(Ordering::Less);
                    } else if a > b {
                        return Some(Ordering::Greater);
                    } else {
                        continue;
                    }
                }
                Some(Ordering::Equal)
            }
            None => None,
        }
    }
}

impl Default for Fishbone {
    fn default() -> Self {
        Self::new()
    }
}

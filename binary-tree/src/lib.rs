#![allow(dead_code)]

// An ordered collection of `T`s.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[test]
fn binary_tree_size() {
    use std::mem::size_of;

    let word = size_of::<usize>();
    assert_eq!(size_of::<BinaryTree<String>>(), word);
    type Triple = (&'static str, BinaryTree<&'static str>, BinaryTree<&'static str>);
    assert_eq!(size_of::<Triple>(), 4 * word);
}

#[test]
fn build_binary_tree() {
    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));

    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));

    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    assert_eq!(tree.walk(),
               vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
}

impl<T: Clone> BinaryTree<T> {
    fn walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.walk();
                result.push(boxed.element.clone());
                result.extend(boxed.right.walk());
                result
            }
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[test]
fn test_add_method_1() {
    let planets = vec!["Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus"];
    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.add(planet);
    }

    assert_eq!(tree.walk(),
               vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
}

#[test]
fn test_add_method_2() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    for planet in vec!["Mars", "Jupiter", "Saturn", "Uranus"] {
        tree.add(planet);
    }

    assert_eq!(
        tree.walk(),
        vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]
    );
}

// From chapter 15: Iterators

use self::BinaryTree::*;

// The state of an in-order traversal of a `BinaryTree`.
struct TreeIter<'a, T> {
    // A stack of references to tree nodes. Since we use `Vec`'s
    // `push` and `pop` methods, the top of the stack is the end of the
    // vector.
    //
    // The node the iterator will visit next is at the top of the stack,
    // with those ancestors still unvisited below it. If the stack is empty,
    // the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce,
        // or finish the iteration. (Use the `?` operator
        // to return immediately if it's `None`.)
        let node = self.unvisited.pop()?;

        // After `node`, the next thing we produce must be the leftmost
        // child in `node`'s right subtree, so push the path from here
        // down. Our helper method turns out to be just what we need.
        self.push_left_edge(&node.right);

        // Produce a reference to this node's value.
        Some(&node.element)
    }
}

#[test]
fn external_iterator() {
    fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>)
               -> BinaryTree<T>
    {
        NonEmpty(Box::new(TreeNode { left, element, right }))
    }

    // Build a small tree.
    let mut tree = BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");

    // Iterate over it.
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);

    assert_eq!(tree.iter()
               .map(|name| format!("mega-{}", name))
               .collect::<Vec<_>>(),
               vec!["mega-droid", "mega-jaeger",
                    "mega-mecha", "mega-robot"]);

    let mut iterator = (&tree).into_iter();
    assert_eq!(iterator.next(), Some(&"droid"));
    assert_eq!(iterator.next(), Some(&"jaeger"));
    assert_eq!(iterator.next(), Some(&"mecha"));
    assert_eq!(iterator.next(), Some(&"robot"));
    assert_eq!(iterator.next(), None);

    // Construct a tree by hand.
    let left_subtree = make_node(Empty, "mecha", Empty);
    let right_subtree = make_node(make_node(Empty, "droid", Empty),
                                  "robot",
                                  Empty);
    let tree = make_node(left_subtree, "Jaeger", right_subtree);

    // Try initializing the iterator ourselves and see if it runs.
    let mut v = Vec::new();
    let mut iter = TreeIter { unvisited: vec![] };
    iter.push_left_edge(&tree);
    for kind in iter {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    // Iterate by shared reference.
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    // Iterate, taking ownership.
    let mut v = Vec::new();
    let mut state = tree.into_iter();
    while let Some(kind) = state.next() {
        v.push(*kind);
    }
    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);
}


#[test]
fn other_cloned() {
    use std::collections::BTreeSet;

    let mut set = BTreeSet::new();
    set.insert("mecha");
    set.insert("Jaeger");
    set.insert("droid");
    set.insert("robot");
    assert_eq!(set.iter().cloned().collect::<Vec<_>>(),
               ["Jaeger", "droid", "mecha", "robot"]);
}

#[test]
fn fuzz() {
    fn make_random_tree(p: f32) -> BinaryTree<i32> {
        use rand::prelude::*;
        use rand::thread_rng;
        use rand::rngs::ThreadRng;

        fn make(p: f32, next: &mut i32, rng: &mut ThreadRng) -> BinaryTree<i32> {
            if rng.gen_range(0.0 .. 1.0) > p {
                Empty
            } else {
                let left = make(p * p, next, rng);
                let element = *next;
                *next += 1;
                let right = make(p * p, next, rng);
                NonEmpty(Box::new(TreeNode { left, element, right }))
            }
        }

        make(p, &mut 0, &mut thread_rng())
    }

    for _ in 0..100 {
        let tree = make_random_tree(0.9999);
        assert!(tree.into_iter().fold(Some(0), |s, &i| {
            s.and_then(|expected| if i == expected { Some(expected+1) } else { None })
        }).is_some());
    }
}

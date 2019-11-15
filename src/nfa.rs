use std::collections::{HashMap, VecDeque, HashSet};
use std::borrow::Cow;
use crate::token::Token;
use dot;

/// NFA data structure contained the start point id,
/// and use the hashmap to map the point to a NFANode.
pub struct NFA {
    start : u64,
    termnial : u64,
    nodes : HashMap<u64, NFANode>,
}

type Nd = u64;
type Ed = (u64, u64, Token);

impl NFA {
    pub fn new(start : u64, termnial : u64) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(start, NFANode::new(start));
        NFA {
            start,
            termnial,
            nodes
        }
    }

    pub fn get_terminal(&self) -> u64 {
        self.termnial
    }
    
    /// get NFA.start
    pub fn get_start(&self) -> u64 {
        self.start 
    }

    /// get NFA.nodes[nodeid]
    pub fn get_node(&self, nodeid : u64) -> Option<&NFANode> {
        self.nodes.get(&nodeid)
    }

    /// push the (from, (Token, to)) tuple to insert it into the NFA graph
    pub fn push(&mut self, nodeid : u64, tuple : (Token, u64)) {
        if self.nodes.get(&tuple.1).is_none() {
            self.nodes.insert(tuple.1, NFANode::new(nodeid));
        }
        if let Some(node) = self.nodes.get_mut(&nodeid) {
            node.push(tuple);
        } else {
            let mut node = NFANode::new(nodeid);
            node.push(tuple);
            self.nodes.insert(nodeid, node);
        }
    }

    /// get all of the epsilon_closures of specified nodeid, which include itself.
    /// return type is Vector, so if you want to translate to other type, please use iter
    pub fn get_epsilon_closure_node(&self, nodeid : u64) -> Option<Vec<u64>> {
        let mut queue = VecDeque::new();
        let mut ans : Vec<u64> = Vec::new();
        let mut visited = HashSet::new();

        queue.push_back(nodeid);
        visited.insert(nodeid);
        ans.push(nodeid);
        
        while !queue.is_empty() {
            let tmp_nodeid = queue.pop_front().unwrap();
            
            let _ : Vec<_> = self.nodes.get(&tmp_nodeid)
                .expect("unkown nodeid")
                .epsilon_closures()
                .iter()
                .map(|id| {
                    if let None = visited.get(id) {
                        ans.push(*id);
                        queue.push_back(*id);
                        visited.insert(*id);
                    }
                })
                .collect();
        }

        Some(ans)
    }
}


impl<'a> dot::GraphWalk<'a, Nd, Ed> for NFA {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let nodes : Vec<Nd> = self.nodes.keys().cloned().collect();
        Cow::Owned(nodes)
    } 

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let edges : Vec<Ed> = self.nodes.values()
            .flat_map(|node| {
                let id = node.nodeid;
                node.edges.iter().map(move |edge| (id, edge.1, edge.0.clone()))
            })
            .collect();

        Cow::Owned(edges)
    }

    fn source(&self, e : &Ed) -> Nd { e.0 }
    fn target(&self, e : &Ed) -> Nd { e.1 }
}

impl<'a> dot::Labeller<'a, Nd, Ed> for NFA {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("NFA").unwrap()
    }

    fn node_id(&'a self, n : &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("node{}", *n)).unwrap()
    }

    fn edge_label(&self, ed : &Ed) -> dot::LabelText {
        let s = match ed.2 {
            Token::Epsilon => "Epsilon".to_string(),
            Token::Character(ch) => ch.to_string(),
            _ => unreachable!(),
        };
        dot::LabelText::LabelStr(Cow::Owned(format!("{}", s)))
    }
}

#[test]
fn test_draw_automachine() {
    let mut nfa = NFA::new(0, 9);
    use std::fs::File;
    let mut output = File::create("example1.dot").unwrap();
    nfa.push(0, (Token::Character('a'), 1));
    nfa.push(1, (Token::Epsilon, 2));
    nfa.push(2, (Token::Epsilon, 3));
    nfa.push(2, (Token::Epsilon, 9));
    nfa.push(3, (Token::Epsilon, 4));
    nfa.push(3, (Token::Epsilon, 6));
    nfa.push(4, (Token::Character('b'), 5));
    nfa.push(5, (Token::Epsilon, 8));
    nfa.push(6, (Token::Character('c'), 7));
    nfa.push(7, (Token::Epsilon, 8));
    nfa.push(8, (Token::Epsilon, 3));
    nfa.push(8, (Token::Epsilon, 9));
    nfa.push(9, (Token::Epsilon, 9));
    dot::render(&nfa, &mut output).unwrap();
}

#[test]
fn test_get_spsilion_closure_node() {
    let mut nfa = NFA::new(0, 9);
    nfa.push(0, (Token::Character('a'), 1));
    nfa.push(1, (Token::Epsilon, 2));
    nfa.push(2, (Token::Epsilon, 3));
    nfa.push(2, (Token::Epsilon, 9));
    nfa.push(3, (Token::Epsilon, 4));
    nfa.push(3, (Token::Epsilon, 6));
    nfa.push(4, (Token::Character('b'), 5));
    nfa.push(5, (Token::Epsilon, 8));
    nfa.push(6, (Token::Character('c'), 7));
    nfa.push(7, (Token::Epsilon, 8));
    nfa.push(8, (Token::Epsilon, 3));
    nfa.push(8, (Token::Epsilon, 9));
    nfa.push(9, (Token::Epsilon, 9));
    println!("0 test result : {:?}", nfa.get_epsilon_closure_node(0));
    println!("1 test result : {:?}", nfa.get_epsilon_closure_node(1));
    println!("2 test result : {:?}", nfa.get_epsilon_closure_node(2));
    println!("3 test result : {:?}", nfa.get_epsilon_closure_node(3));
    println!("4 test result : {:?}", nfa.get_epsilon_closure_node(4));
    println!("5 test result : {:?}", nfa.get_epsilon_closure_node(5));
    println!("6 test result : {:?}", nfa.get_epsilon_closure_node(6));
    println!("7 test result : {:?}", nfa.get_epsilon_closure_node(7));
    println!("8 test result : {:?}", nfa.get_epsilon_closure_node(8));
    println!("9 test result : {:?}", nfa.get_epsilon_closure_node(9));
}

pub struct NFANode {
    nodeid : u64,
    edges : Vec<(Token, u64)>,
}

impl NFANode {
    pub fn get_edges(&self) -> &Vec<(Token, u64)> {
        &self.edges
    }
    fn new(nodeid : u64) -> Self {
        NFANode {
            nodeid,
            edges : Vec::new(),
        } 
    }
    fn push(&mut self, tuple : (Token, u64)) {
        self.edges.push(tuple);
    }
    fn epsilon_closures(&self) -> Vec<u64> {
        self.edges
            .iter()
            .filter(|tuple| tuple.0 == Token::Epsilon)
            .filter_map(|tuple| {
                if tuple.0 == Token::Epsilon {
                    Some(tuple.1)
                } else {
                    None
                }
            })
            .collect()
    }
}


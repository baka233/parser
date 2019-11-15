use std::collections::{HashSet, VecDeque,HashMap, BTreeSet, BTreeMap};
use std::borrow::Cow;
use crate::nfa::NFA;
use crate::token::Token;
use dot;

pub struct DFA {
    start : u64,
    nodes : HashMap<u64, DFANode>,
    /// point_map only contains the NFA node id, we use it only to verify 
    /// from_nfa algorithm
    id_to_point : HashMap<u64, Vec<u64>>
}

impl DFA {
    pub fn print(&self) {
        println!("start is {}", self.start);
        for (key, value) in self.id_to_point.iter() {
            println!("node{} is {:?}, type is {:?}", key, value, self.nodes.get(key).unwrap().nodetype);
        }
    }

    pub fn from_nfa(nfa : &NFA) -> Self {
        let mut worklist = VecDeque::new();
        let mut point_map : HashMap<Vec<u64>, u64> = HashMap::new();
        let mut id_to_point : HashMap<u64, Vec<u64>> = HashMap::new();
        let mut nodes : HashMap<u64, DFANode> = HashMap::new();
        let mut num = 0;
        let mut nodetype = DFANodeType::NonTerminal;
        let terminal = nfa.get_terminal();

        let mut tmp = nfa.get_epsilon_closure_node(nfa.get_start()).unwrap();
        if tmp.contains(&terminal) {
            nodetype = DFANodeType::Terminal;
        }
        tmp.sort();
        point_map.insert(tmp.clone(), num);
        id_to_point.insert(num, tmp.clone());
        worklist.push_back(num);
        nodes.insert(num, DFANode::new(num, nodetype));
        
        while !worklist.is_empty() {
            let q_id = worklist.pop_front().unwrap();
            let q = id_to_point.get(&q_id).unwrap();
            let mut tmp : HashMap<Token, BTreeSet<u64>> = HashMap::new();


            for node in q {
                nfa.get_node(*node)
                    .unwrap()
                    .get_edges()
                    .iter()
                    .for_each(|tuple| {
                        if tuple.0 == Token::Epsilon {
                            return;
                        }
                        if tmp.get(&tuple.0).is_none() {
                            tmp.insert(tuple.0.clone(), BTreeSet::new());
                        }

                        let tmp_set = tmp.get_mut(&tuple.0).unwrap();

                        nfa.get_epsilon_closure_node(tuple.1).unwrap()
                            .iter()
                            .for_each(|data| {
                                tmp_set.insert(*data);
                            })
                    });
            }

            for (key, value) in tmp.iter_mut() {
                let item : Vec<u64> = value.iter().cloned().collect();
                nodetype = DFANodeType::NonTerminal;
                if item.contains(&terminal) {
                    nodetype = DFANodeType::Terminal;
                }
                if point_map.get(&item).is_none() {
                    num += 1;
                    point_map.insert(item.clone(), num);
                    id_to_point.insert(num, item.clone());
                    nodes.insert(num, DFANode::new(num, nodetype));
                    worklist.push_back(num);
                }
                let q_node = nodes.get_mut(&q_id).unwrap();
                q_node.push(key.clone(), *point_map.get(&item).unwrap());
            }
        }

        
        DFA {
            start : 0,
            nodes,
            id_to_point
        }
    }

    pub fn scanner(&self, string : &str) -> String {
        let mut ans = String::new();
        let mut max_ans = String::new();
        let mut id = self.start;
        let mut iter = string.chars();
        let mut ch = 'a';
        let mut pos_flag = true;
        loop {
            if pos_flag {
                match iter.next() {
                    Some(c) => ch = c,
                    None => break,
                }
            } else {
                pos_flag = !pos_flag;
            }
            let token = Token::Character(ch);    
            let mut flag = false;
            if self.nodes.get(&id).unwrap().nodetype == DFANodeType::Terminal {
                if max_ans.len() < ans.len() {
                    max_ans = ans.clone(); 
                }
            }
            for (test_token, to) in self.nodes.get(&id)
                .expect("unknown id to scanned")
                .edges
                .iter() 
            {
                //println!("edge is {:?} {}, current is {:?}", test_token, to, token);
                if *test_token == token {
                    id = *to;
                    flag = true;
                    break;
                } 
            }

            if !flag {
                pos_flag = !pos_flag;
                //println!("has been cleared, {}", pos_flag);
                id = self.start;
                ans.clear();
            } else {
                //println!("{} to {}", ch, ans);
                ans.push(ch);
            }
        }

        max_ans
    }

    pub fn simplifier(&mut self) {
        let mut groups = Vec::new();
        let mut map = BTreeMap::new();
        let mut num = 1;
        let mut nodes = HashMap::new();
        let mut id_to_point = HashMap::new();
        let mut start = 0;
        groups.push(BTreeSet::new());
        groups.push(BTreeSet::new());
        self.nodes
            .iter()
            .for_each(|(key, value)| {
                if value.nodetype == DFANodeType::Terminal {
                    groups.get_mut(0).unwrap().insert(*key);
                    map.insert(*key, 0); 
                } else {
                    groups.get_mut(1).unwrap().insert(*key);
                    map.insert(*key, 1);
                }
            });


        let mut pre_size = 0;
        loop {
            let size = groups.len();
            if size == pre_size {
                break;
            }
            for i in 0..size {
                let mut to_type : HashMap<Vec<(Token, u64)>, Vec<u64>> = HashMap::new();
                //let first_element = top.pop_front().expect("the input file didn't have start and end");
                //{
                //    let mut tmp_vec = Vec::new();
                //    tmp_vec.push(first_element);
                //    let mut first_element_to_type : Vec<_> = self.nodes
                //        .get(&first_element)
                //        .unwrap()
                //        .edges
                //        .iter()
                //        .map(|(token, id)| {
                //            println!("id {} to {}, {:?}", first_element,  *map.get(id).unwrap(), token);
                //            (token.clone(), *map.get(id).unwrap())
                //        })
                //        .collect();
                //    first_element_to_type.sort();

                //    to_type.insert(first_element_to_type, tmp_vec);
                //}
                
                for other in groups.get_mut(i).unwrap().iter() {
                    let mut other_to_type : Vec<_> = self.nodes
                        .get(&other)
                        .unwrap()
                        .edges
                        .iter()
                        .map(|(token, id)| {
                            println!("id {} to {}, {:?}", other,  *map.get(id).unwrap(), token);
                            (token.clone(), *map.get(id).unwrap())
                        })
                        .collect();
                    other_to_type.sort();
                    if to_type.get(&other_to_type).is_none() {
                        let mut tmp_vec = Vec::new();
                        tmp_vec.push(*other);
                        to_type.insert(other_to_type.clone(), tmp_vec);
                    } else {
                        to_type.get_mut(&other_to_type).unwrap().push(*other);
                    }
                }
                
                let mut flag = true;
                for (key, value) in &to_type {
                    if flag {
                        let gen = i;
                        flag = false;
                        continue;
                    }
                    groups.push(value.iter().cloned().collect());
                    value.iter()
                        .for_each(|item| {
                            map.insert(*item, groups.len() as u64 - 1);                    
                            groups.get_mut(i).unwrap().remove(item);
                        });
                }
            }   
            pre_size = size;
        }

        for i in 0..groups.len() { 
            if groups.get(i).unwrap().len() == 0 {
                continue;
            }
            let mut nodetype = DFANodeType::NonTerminal;
            let to_insert : Vec<_> = groups.get(i).unwrap()
                .iter()
                .map(|item| {
                    if self.nodes.get(item).unwrap().nodetype == DFANodeType::Terminal {
                        nodetype = DFANodeType::Terminal;
                    }
                    if *item == self.start {
                        start = i;
                    }
                    item
                })
                //.flat_map(|item| {
                //    print!("{} ", item);
                //    self.id_to_point.get(item).unwrap()
                //})
                .cloned()
                .collect();
            id_to_point.insert(i as u64, to_insert);
            let mut node = DFANode::new(i as u64, nodetype);
            self.nodes.get(groups.get(i).unwrap().iter().next().unwrap())
                .unwrap()
                .edges
                .iter()
                .for_each(|(token, to)| {
                    node.push(token.clone(), map.get(to).unwrap().clone());
                });
            nodes.insert(i as u64, node);
        }



        self.start = start as u64;
        self.nodes = nodes;
        self.id_to_point = id_to_point;
        
        
        
    }

}

type Nd = u64;
type Ed = (u64, u64, Token);


impl<'a> dot::GraphWalk<'a, Nd, Ed> for DFA {
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

impl<'a> dot::Labeller<'a, Nd, Ed> for DFA {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DFANodeType {
    Terminal,
    NonTerminal,
}

struct DFANode {
    nodeid : u64,
    nodetype : DFANodeType,
    edges : Vec<(Token, u64)>, 
}

impl DFANode {
    fn new(nodeid : u64, nodetype : DFANodeType) -> Self {
        DFANode {
            nodeid,
            nodetype,
            edges : Vec::new(),
        }
    }

    fn push(&mut self, token : Token, to : u64) {
        self.edges.push((token, to));
    }
}




#[test]
fn test_nfa_to_dfa_simplifier_1() {
    let mut nfa = test_base_nfa_1();
    let mut dfa = DFA::from_nfa(&nfa);
    use std::fs::File;
    let mut output = File::create("simplifier1.dot").unwrap();
    dfa.print();
    dfa.simplifier();
    dot::render(&dfa, &mut output).unwrap();
    dfa.print();
}

#[test]
fn test_nfa_to_dfa_simplifier_2() {
    let mut nfa = test_base_nfa_2();
    let mut dfa = DFA::from_nfa(&nfa);
    use std::fs::File;
    let mut output = File::create("simplifier2.dot").unwrap();
    dfa.print();
    dfa.simplifier();
    dot::render(&dfa, &mut output).unwrap();
    dfa.print();
}

#[test]
fn test_draw_dfa_automachine() {
    use std::fs::File;
    let mut output = File::create("example.dot").unwrap();
    let mut nfa = test_base_nfa_1();
    let mut dfa = DFA::from_nfa(&nfa);
    dot::render(&dfa, &mut output).unwrap();
}

#[test]
fn test_draw_dfa_automachine_2() {
    use std::fs::File;
    let mut output = File::create("example2.dot").unwrap();
    let mut nfa = test_base_nfa_2();
    let mut dfa = DFA::from_nfa(&nfa);
    dot::render(&dfa, &mut output).unwrap();
}

#[test]
fn test_nfa_to_dfa() {
    let mut nfa = test_base_nfa_1();
    let dfa = DFA::from_nfa(&nfa);
    dfa.print();
}

#[test]
fn test_nfa_to_dfa_2() {
    let mut nfa = test_base_nfa_2(); 
    let dfa = DFA::from_nfa(&nfa);
    dfa.print();
}

fn test_base_nfa_2() -> NFA{
    let mut nfa = NFA::new(0, 6);
    nfa.push(0, (Token::Epsilon, 5));
    nfa.push(5, (Token::Epsilon, 1));
    nfa.push(5, (Token::Character('a'), 5));
    nfa.push(5, (Token::Character('b'), 5));
    nfa.push(1, (Token::Character('a'), 3));
    nfa.push(1, (Token::Character('b'), 4));
    nfa.push(3, (Token::Character('a'), 2));
    nfa.push(4, (Token::Character('b'), 2));
    nfa.push(2, (Token::Epsilon, 6));
    nfa.push(6, (Token::Character('a'), 6));
    nfa.push(6, (Token::Character('b'), 6));
    nfa.push(6, (Token::Epsilon, 7));
    nfa
}

fn test_base_nfa_1() -> NFA {
    let mut nfa = NFA::new(0, 9);
    use std::fs::File;
    let mut output = File::create("example2.dot").unwrap();
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
    nfa
}

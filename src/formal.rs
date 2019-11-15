use std::collections::{HashSet, HashMap, VecDeque};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use crate::token::Token;
use crate::nfa::NFA;
use crate::dfa::DFA;

pub struct FormalLanguage {
    start : Token,
    grammer : HashMap<Token, Vec<String>>,
}

impl FormalLanguage {
    pub fn new(start : Token, grammer : HashMap<Token, Vec<String>>) -> Self {
        FormalLanguage {
            start,
            grammer,
        } 
    }

    pub fn iter(&self) -> FormalLanguageIterator {
        FormalLanguageIterator::new(&self.grammer, self.start.clone())       
    }

    pub fn get_nfa(&self) -> NFA{
        let mut nfa = NFA::new(1, 0);
        self.iter()
            .for_each(|tuple| {
                nfa.push(tuple.0, (tuple.2, tuple.1))
            });
        nfa
    }

    pub fn print(&self) {
        self.iter()
            .for_each(|tuple| {
                println!("{:?}", tuple);
            })
    }

    pub fn get_grammer<R>(reader : &mut R) -> HashMap<Token, Vec<String>> 
    where 
        R : BufRead
    {
        let mut map : HashMap<Token, Vec<String>> = HashMap::new();
        let mut buf = String::new();
        while reader.read_line(&mut buf).unwrap() != 0 {
            let data : Vec<_> = buf.trim()
                .split("->")
                .map(str::trim)
                .collect();
            println!("{:?}", data);
            if data.len() != 2 {
                panic!("wrong grammer!");
            }

            let token = Token::Identifier(data[0].chars().next().unwrap());
            
            if let Some(nodes) = map.get_mut(&token) {
                nodes.push(data[1].to_string());
            } else {
                let mut nodes = Vec::new();
                nodes.push(data[1].to_string());
                map.insert(token, nodes);
            }
            buf.clear();
        }

        map
    }


}

pub struct FormalLanguageIterator<'a> {
    tmp : Token,
    grammer : &'a HashMap<Token, Vec<String>>,
    pos : Option<u64>,
    token_map : HashMap<Token, u64>,
    visited : HashSet<Token>,
    queue : VecDeque<Token>,
    num : u64,
    end : u64, 
}

impl<'a> FormalLanguageIterator<'a> {
    fn new(grammer : &'a HashMap<Token, Vec<String>>, start : Token) -> Self {
        let mut token_map = HashMap::new();
        let mut visited  = HashSet::new(); 
        let mut queue = VecDeque::new();
        let num = 1;
        let end = 0;

        token_map.insert(start.clone(), num);
        visited.insert(start.clone());
        queue.push_back(start.clone());

        FormalLanguageIterator {
            tmp : start,
            grammer,
            token_map,
            visited,
            pos : None,
            queue,
            num,
            end,
        } 
    }
}

impl<'a> Iterator for FormalLanguageIterator<'a> {
    type Item = (u64, u64, Token);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos.is_none() || self.grammer.get(&self.tmp)
            .expect("unknown grammer")
            .get(self.pos.unwrap() as usize).is_none()
        {
            if !self.queue.is_empty() {
                self.tmp = self.queue.pop_front().unwrap();
                self.pos = Some(0);
            } else {
                return None;
            }
        }
        
        let sentence = self.grammer.get(&self.tmp)
            .unwrap()
            .get(self.pos.unwrap() as usize).unwrap();


        let tmpid = *self.token_map.get(&self.tmp).unwrap();
        let mut iter = sentence.chars();
        let first = iter.next().expect("wrong sentence");
        if let Some(second) = iter.next() {
            let token = Token::Identifier(second);
            if let None = self.visited.get(&token) {
                self.visited.insert(token.clone());
                self.queue.push_back(token.clone());
            }
            let to = match self.token_map.get(&token) {
                Some(t) => *t,
                None => {
                    self.num += 1;
                    self.token_map.insert(token, self.num);
                    self.num
                }
            };
            self.pos = Some(self.pos.unwrap() + 1);
            return Some((tmpid, to, Token::Character(first)));
        } else {
            self.pos = Some(self.pos.unwrap() + 1);
            if first == 'ε' {
                return Some((tmpid, self.end, Token::Epsilon));
            } else {
                return Some((tmpid, self.end, Token::Character(first)));
            }
        }
    }
}



#[test]
fn test_normal_language() {
    let data = FormalLanguage::get_grammer(&mut BufReader::new(&mut File::open("test.in").unwrap()));
    let formal = FormalLanguage::new(Token::Identifier('S'), data);
    formal.print();
    let nfa = formal.get_nfa();
    println!("0 test result : {:?}", nfa.get_epsilon_closure_node(0));
    println!("1 test result : {:?}", nfa.get_epsilon_closure_node(1));
    println!("2 test result : {:?}", nfa.get_epsilon_closure_node(2));
    println!("3 test result : {:?}", nfa.get_epsilon_closure_node(3));

}


#[test]
fn test_dfa() {
    let data = FormalLanguage::get_grammer(&mut BufReader::new(&mut File::open("test2.in").unwrap()));
    let formal = FormalLanguage::new(Token::Identifier('S'), data);
    formal.print();
    let nfa = formal.get_nfa();
    let mut dfa = DFA::from_nfa(&nfa);
    dfa.simplifier();
    dfa.print();
    println!("match pattern to {} is {}", "aaaabcaab", dfa.scanner("aaaabcaab"));
}

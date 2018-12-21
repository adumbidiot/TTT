#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::any::Any;

pub mod ttt;
//pub mod ttt2x2;

use std::collections::HashMap;
use std::collections::VecDeque;

pub type NodeIndex = u128;
pub type NodeMap = HashMap<NodeIndex, Node>;

pub trait Compilation {
	fn inc_nodes_processed(&mut self);
	fn get_nodes_processed(&self) -> usize;
	
	fn inc_winners_processed(&mut self);
	fn get_winners_processed(&self) -> usize;
	
	fn inc_nodes_scored(&mut self);
	fn get_nodes_scored(&self) -> usize;
	
	fn get_node_mut(&mut self, id: NodeIndex) -> &mut Node;
	fn insert_node(&mut self, id: NodeIndex, n: Node);
	fn contains_node(&self, id: &NodeIndex) -> bool;
	
	fn get_cloned_map(&self) -> NodeMap;
	
	fn get_winner(&self, id: &NodeIndex) -> u8;
	
	fn get_child_states(&self, id: NodeIndex, team: u8) -> Vec<NodeIndex>; 
	
	fn reset(&mut self);
	
	fn as_any(&mut self) -> &mut Any;
}

pub struct Compiler {
	pub queue: VecDeque<NodeIndex>,
	pub winners: VecDeque<NodeIndex>,
	pub unscored_nodes: Vec<NodeIndex>,
	pub compilation: Option<Box<Compilation>>
}

impl Compiler {
	pub fn new() -> Compiler {
		let compiler = Compiler {
			queue: VecDeque::new(),
			winners: VecDeque::new(),
			unscored_nodes: Vec::new(),
			compilation: None
		};
		
		return compiler;
	}
	
	pub fn init_compilation(&mut self){
		self.create_node(0, 0);
		self.queue.push_back(0);
	}
	
	pub fn create_node(&mut self, id: NodeIndex, level: usize){
		let mut n = Node::new();
		n.id = id;
		n.level = level;
		
		self.compilation.as_mut().unwrap().insert_node(id, n);
	}
	
	pub fn is_node(&self, id: &NodeIndex) -> bool {
		return self.compilation.as_ref().unwrap().contains_node(id);
	}
	
	pub fn get_node(&mut self, index: NodeIndex) -> &mut Node {
		return self.compilation.as_mut().unwrap().get_node_mut(index);
	}
	
	pub fn get_child_states(&self, id: NodeIndex, team: u8) -> Vec<NodeIndex> {
		return self.compilation.as_ref().unwrap().get_child_states(id, team);
	}
	
	pub fn get_winner(&self, index: &NodeIndex) -> u8 {
		return self.compilation.as_ref().unwrap().get_winner(index);
	}
	
	pub fn process(&mut self){
		if self.queue.len() == 0 {
			return;
		}
		
		let node_id = self.queue.pop_front().unwrap();
		let node_level = self.get_node(node_id).level;
		
		let team: u8 = (self.get_node(node_id).level % 2) as u8 + 1;
		
		self.compilation.as_mut().unwrap().inc_nodes_processed();
		
		if self.get_winner(&node_id) != 0 {
			return; //Can't keep playing after someone has won, no need to process
		}else{
			self.unscored_nodes.push(node_id);
		}
		
		let states = self.get_child_states(node_id, team);
		
		for i in 0..states.len() {
			if !self.is_node(&states[i]){
				//We discovered a new Node!
				self.create_node(states[i], node_level + 1); //Setup the node..
				self.queue.push_back(states[i]); //and set it to be processed some time in the future.
				//Since its new, we can check to see if its a "winner"
				if self.get_winner(&states[i]) != 0 {
					//It is!
					self.winners.push_back(states[i]); //Save to score it later
				}
			}
				
			self.get_node(states[i]).parents.push(node_id);
			self.get_node(node_id).children.push(states[i]);
		}
	}
	
	pub fn post_process(&mut self){
		if self.winners.len() == 0 {
			return;
		}
		
		let node_id = self.winners.pop_front().unwrap();
		let winner = self.get_winner(&node_id);
		let score = if winner == 1 {
			100
		}else{
			-100
		};
		
		self.get_node(node_id).score = score as i8;
		self.compilation.as_mut().unwrap().inc_winners_processed();
	}
	
	pub fn score_nodes(&mut self){
		if self.unscored_nodes.len() == 0 {
			return;
		}
		
		let node_id = self.unscored_nodes.pop().unwrap();
		
		let mut scores = Vec::new();
		
		let it = self.get_node(node_id).children.clone();
		
		for child_id in it.iter() {
			scores.push(self.get_node(child_id.clone()).score);
		}
		
		if scores.len() == 0 {
			scores.push(0);
		}
		
		let score = if self.get_node(node_id).level % 2 == 0 {
			scores.iter().max().unwrap()
		}else{
			scores.iter().min().unwrap()
		};
		
		self.get_node(node_id).score = score.clone();
		
		self.compilation.as_mut().unwrap().inc_nodes_scored();
	}
	
	pub fn export(&self) -> HashMap<String, Node>{
		let map = self.compilation.as_ref().unwrap().get_cloned_map();
		let mut export_map: HashMap<String, Node> = HashMap::new();
		
		for (k, v) in map {
			export_map.insert(k.to_string(), v); //u128 doesn't serialize correctly with serde, and js can't handle u128s anyway
		}
		
		return export_map;
	}
	
	pub fn get_nodes_processed(&self) -> usize{
		return self.compilation.as_ref().unwrap().get_nodes_processed();
	}
	
	pub fn get_winners_processed(&self) -> usize {
		return self.compilation.as_ref().unwrap().get_winners_processed();
	}
	
	pub fn get_nodes_scored(&self) -> usize {
		return self.compilation.as_ref().unwrap().get_nodes_scored();
	}
}

pub struct AI {
	nodes: NodeMap,
}

impl AI {
	pub fn new() -> AI { 
		AI {
			nodes: NodeMap::new(),
		}
	}
	
	pub fn load(&mut self, n: NodeMap){
		self.nodes = n;
	}
	
	pub fn get_node(&self, id: &NodeIndex) -> &Node {
		return self.nodes.get(id).unwrap();
	}
	
	pub fn get_move(&self, id: NodeIndex, team: u8) -> NodeIndex{
		let node = self.get_node(&id);
		
		let mut child_id = node.children[0];
		for i in 0..node.children.len(){
			if team == 1 && self.get_node(&child_id).score < self.get_node(&node.children[i]).score {
				child_id = node.children[i];
			}else if team == 2 && self.get_node(&child_id).score > self.get_node(&node.children[i]).score{
				child_id = node.children[i];
			}
		}
		
		return child_id;
	}
	
	pub fn get_score(&self, id: &NodeIndex) -> i8{
		return self.get_node(id).score;
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
	pub id: NodeIndex,
	level: usize,
	parents: Vec<NodeIndex>,
	pub children: Vec<NodeIndex>,
	pub score: i8,
}

impl Node {
	fn new() -> Node {
		return Node {
			id: 0,
			level: 0,
			parents: Vec::new(),
			children: Vec::new(),
			score: 0
		};
	}
}

#[cfg(test)]
mod tests;
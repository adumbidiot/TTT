#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::any::Any;
use std::collections::{HashMap, VecDeque};

pub mod ttt;
//pub mod ttt2x2;

pub type NodeIndex = u128;
pub type NodeMap = HashMap<NodeIndex, Node>;
pub type CompilationResult<T> = Result<T, CompilationError>;

#[derive(Debug)]
pub enum CompilationError {
	NoCompilation,
	QueueEmpty
}

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
		Compiler {
			queue: VecDeque::new(),
			winners: VecDeque::new(),
			unscored_nodes: Vec::new(),
			compilation: None
		}
	}
	
	pub fn init_compilation(&mut self) -> CompilationResult<()>{
		self.create_node(0, 0)?;
		self.queue.push_back(0);
		return Ok(());
	}
	
	pub fn create_node(&mut self, id: NodeIndex, level: usize) -> CompilationResult<()>{
		let mut n = Node::new();
		n.id = id;
		n.level = level;
		
		return self.get_compilation_mut().map(|compilation| compilation.insert_node(id, n));
	}
	
	pub fn is_node(&self, id: &NodeIndex) -> CompilationResult<bool>{
		return self.get_compilation().map(|compilation| compilation.contains_node(id));
	}
	
	pub fn get_node(&mut self, index: NodeIndex) -> CompilationResult<&mut Node>{
		return self.get_compilation_mut().map(|compilation| compilation.get_node_mut(index));
	}
	
	pub fn get_child_states(&self, id: NodeIndex, team: u8) -> CompilationResult<Vec<NodeIndex>>{
		return self.get_compilation().map(|compilation| compilation.get_child_states(id, team));
	}
	
	pub fn get_winner(&self, index: &NodeIndex) -> CompilationResult<u8>{
		return self.get_compilation().map(|compilation| compilation.get_winner(index));
	}
	
	pub fn process(&mut self) -> CompilationResult<()>{		
		let node_id = self.queue.pop_front().ok_or(CompilationError::QueueEmpty)?;
		let node_level = self.get_node(node_id)?.level;
		
		let team: u8 = (node_level % 2) as u8 + 1;
		
		self.get_compilation_mut()?.inc_nodes_processed();
		
		if self.get_winner(&node_id)? != 0 {
			return Ok(()); //Can't keep playing after someone has won, no need to process
		}else{
			self.unscored_nodes.push(node_id);
		}
		
		let states = self.get_child_states(node_id, team)?;
		
		for i in 0..states.len() {
			if !self.is_node(&states[i])?{
				//We discovered a new Node!
				self.create_node(states[i], node_level + 1)?; //Setup the node..
				self.queue.push_back(states[i]); //and set it to be processed some time in the future.
				//Since its new, we can check to see if its a "winner"
				if self.get_winner(&states[i])? != 0 {
					//It is!
					self.winners.push_back(states[i]); //Save to score it later
				}
			}
				
			self.get_node(states[i])?.parents.push(node_id);
			self.get_node(node_id)?.children.push(states[i]);
		}
		
		return Ok(());
	}
	
	pub fn post_process(&mut self) -> CompilationResult<()>{
		let node_id = self.winners.pop_front().ok_or(CompilationError::QueueEmpty)?;
		let score = if self.get_winner(&node_id)? == 1 {
			100
		}else{
			-100
		};
		
		self.get_node(node_id)?.score = score as i8;
		self.get_compilation_mut()?.inc_winners_processed();
		return Ok(());
	}
	
	pub fn score_nodes(&mut self) -> CompilationResult<()>{		
		let node_id = self.unscored_nodes.pop().ok_or(CompilationError::QueueEmpty)?;
		
		let mut scores = Vec::new();
		
		let it = self.get_node(node_id)?.children.clone();
		
		for child_id in it.iter() {
			scores.push(self.get_node(child_id.clone())?.score);
		}
		
		if scores.len() == 0 {
			scores.push(0);
		}
		
		let score = if self.get_node(node_id)?.level % 2 == 0 {
			scores.iter().max().expect("Finding max scores failed")
		}else{
			scores.iter().min().expect("Finding min scores failed")
		};
		
		self.get_node(node_id)?.score = score.clone();
		
		self.get_compilation_mut()?.inc_nodes_scored();
		return Ok(());
	}
	
	pub fn export(&self) -> CompilationResult<HashMap<String, Node>>{
		let map = self.get_compilation()?.get_cloned_map();
		
		let mut export_map: HashMap<String, Node> = HashMap::new();
		
		for (k, v) in map {
			export_map.insert(k.to_string(), v); //u128 doesn't serialize correctly with serde, and js can't handle u128s anyway
		}
		
		return Ok(export_map);
	}
	
	pub fn get_nodes_processed(&self) -> CompilationResult<usize>{
		return self.get_compilation().map(|compilation| compilation.get_nodes_processed());
	}
	
	pub fn get_winners_processed(&self) -> CompilationResult<usize>{
		return self.get_compilation().map(|compilation| compilation.get_winners_processed());
	}
	
	pub fn get_nodes_scored(&self) -> CompilationResult<usize>{
		return self.get_compilation().map(|compilation| compilation.get_nodes_scored());
	}
	
	pub fn get_compilation(&self) -> CompilationResult<&Box<Compilation>>{
		return self.compilation
			.as_ref()
			.ok_or(CompilationError::NoCompilation);
	}
	
	pub fn get_compilation_mut(&mut self) -> CompilationResult<&mut Box<Compilation>>{
		return self.compilation
			.as_mut()
			.ok_or(CompilationError::NoCompilation);
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
	
	pub fn get_node(&self, id: &NodeIndex) -> Option<&Node>{
		return self.nodes.get(id);
	}
	
	pub fn get_move(&self, id: NodeIndex, team: u8) -> Option<NodeIndex>{
		let node = self.get_node(&id)?;
		
		let mut child_id = node.children[0];
		for i in 0..node.children.len(){
			let node_score = self.get_score(&node.children[i])?;
			let child_score = self.get_score(&child_id)?;
			
			if team == 1 && child_score < node_score{
				child_id = node.children[i];
			}else if team == 2 && child_score > node_score{
				child_id = node.children[i];
			}
		}
		
		return Some(child_id);
	}
	
	pub fn get_score(&self, id: &NodeIndex) -> Option<i8>{
		return self
			.get_node(id)
			.map(|node| node.score);
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
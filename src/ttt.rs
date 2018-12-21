use super::*;

pub struct TTTCompilation{
	pub nodes: NodeMap,
	pub nodes_processed: usize,
	pub winners_processed: usize,
	pub nodes_scored: usize,
	pub board_size: u8,
}

impl TTTCompilation{
	pub fn new() -> TTTCompilation{
		TTTCompilation{
			nodes: NodeMap::new(),
			nodes_processed: 0,
			winners_processed: 0,
			nodes_scored: 0,
			board_size: 3,
		}
	}
	
	pub fn set_board_size(&mut self, size: u8){
		self.board_size = size;
	}
}

impl Compilation for TTTCompilation{
	fn inc_nodes_processed(&mut self){
		self.nodes_processed += 1;
	}
	
	fn get_nodes_processed(&self) -> usize {
		return self.nodes_processed;
	}
	
	fn inc_winners_processed(&mut self){
		self.winners_processed += 1;
	}
	
	fn get_winners_processed(&self) -> usize {
		return self.winners_processed;
	}
	
	fn inc_nodes_scored(&mut self){
		self.nodes_scored += 1;
	}
	
	fn get_nodes_scored(&self) -> usize {
		return self.nodes_scored;
	}
	
	fn get_node_mut(&mut self, id: NodeIndex) -> &mut Node {
		return self.nodes.get_mut(&id).unwrap();
	}
	
	fn insert_node(&mut self, id: NodeIndex, n: Node){
		self.nodes.insert(id, n);
	}
	
	fn contains_node(&self, id: &NodeIndex) -> bool {
		return self.nodes.contains_key(id);
	}
	
	fn get_cloned_map(&self) -> NodeMap {
		return self.nodes.clone();
	}
	
	fn get_winner(&self, id: &NodeIndex) -> u8 {
		return get_winner(id, self.board_size);
	}
	
	fn get_child_states(&self, id: NodeIndex, team: u8) -> Vec<NodeIndex> {
		let mut temp_id = id.clone();
		let mut states = Vec::new();
	
		for i in 0..(self.board_size * self.board_size) as u32 {
			let num = temp_id % 3;
			if num == 0 {
				let three: NodeIndex = 3;
				let new_state = id + (team as NodeIndex * three.pow(i));
				states.push(new_state);
			}
		
			temp_id = temp_id / 3;
		}
	
		return states;
	}

	fn reset(&mut self){
		self.nodes = NodeMap::new();
		self.nodes_processed = 0;
		self.winners_processed = 0;
		self.nodes_scored = 0;
		self.board_size = 3;
	}
	
	fn as_any(&mut self) -> &mut Any{
		self
	}
}

pub fn get_winner(id: &NodeIndex, size: u8) -> u8 {
	let winner = get_winner_row(id, size);
	if winner != 0 {
		return winner;
	}
		
	let winner = get_winner_col(id, size);
	if winner != 0 {
		return winner;
	}
		
	let winner = get_winner_diag(id, size);
	if winner != 0 {
		return winner;
	}
	
	return 0;
}

pub fn get_winner_row(id: &NodeIndex, size: u8) -> u8 {
	let mut id = id.clone();
	let mut team = 0;
	
	for _ in 0..size {
		team = id % 3;
		if team == 0 {
			id = id / 27; //3 ^ 3 = 127
			continue;
		}
		
		for _ in 0..size {
			if team != id % 3 {
				team = 0;
			}
			
			id = id / 3;
		}
		
		if team != 0 {
			break;
		}
	}
	
	return team as u8;
}

pub fn get_winner_col(id: &NodeIndex, size: u8) -> u8 {
	let mut main_id = id.clone();
	let mut team = 0;
	
	for _ in 0..size {
		let mut id = main_id.clone();
		team = id % 3;
		
		if team == 0 {
			main_id = main_id / 3;
			continue;
		}
		
		for _ in 0..size {
			if team != id % 3 {
				team = 0;
			}
			
			id = id / 27;
		}
		
		if team != 0 {
			break;
		}
		
		main_id = main_id / 3;
	}
	
	return team as u8;
}

pub fn get_winner_diag(id: &NodeIndex, size: u8) -> u8 {
	let mut main_id = id.clone();
	let mut team = 0;
	
	for i in 0..2 {
		let mut id = main_id.clone();
		team = id % 3;
		
		if team == 0 {
			main_id = main_id / 9;
			continue;
		}
		
		for _ in 0..size {
			if team != id % 3 {
				team = 0;
			}
			
			id = id / 3u128.pow(4 / (i + 1));
		}
		
		if team != 0 {
			break;
		}
		
		main_id = main_id / 9; //3 ^ 2
	}
	
	return team as u8;	
}
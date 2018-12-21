use super::*;
use super::ttt::*;
use ttt::get_winner_col;
use ttt::get_winner_diag;
use ttt::get_winner_row;
//use super::ttt::*;
#[derive(Clone)]
pub struct State {
	arr: [char; 9]
}


impl State {
	fn new() -> State {
		State {
			arr: ['N'; 9] 
		}
	}
}

impl State {
	fn from(s: &str) -> State{
		let v: Vec<char> = s.chars().collect();
		
		let mut s = State {
			arr: ['N'; 9]
		};
		
		for i in 0..v.len() {
			s.arr[i] = v[i];
		}
		
		return s;
	}
}
	
#[test]
fn winner_row_none() {
	assert_eq!(0, get_winner_row(&0));
}
	
#[test]
fn winner_row_0() {
	let s = State::from("XXXNNNNNN");
	assert_eq!(1, get_winner_row(&hash_state(&s)));
}


#[test]
fn winner_row_1() {
	let s = State::from("NNNXXXNNN");
	assert_eq!(1, get_winner_row(&hash_state(&s)));
}
	
#[test]
fn winner_row_2() {
	let s = State::from("NNNNNNXXX");
	assert_eq!(1, get_winner_row(&hash_state(&s)));
}

#[test]
fn get_winner_row_9641() {
	assert_eq!(1, get_winner_row(&9641));
}

#[test]
fn winner_col_none() {
	assert_eq!(0, get_winner_col(&0));
}

#[test]
fn winner_col_0() {
	let s = State::from("XNNXNNXNN");
	assert_eq!(1, get_winner_col(&hash_state(&s)));
}

#[test]
fn winner_col_1() {
	let s = State::from("NXNNXNNXN");
	assert_eq!(1, get_winner_col(&hash_state(&s)));
}

#[test]
fn winner_col_2() {
	let s = State::from("NNXNNXNNX");
	assert_eq!(1, get_winner_col(&hash_state(&s)));
}


#[test]
fn winner_diag_none(){
	assert_eq!(0, get_winner_diag(&0));
}

#[test]
fn winner_diag_0(){
	let s = State::from("XNNNXNNNX");
	assert_eq!(1, get_winner_diag(&hash_state(&s)));
}


#[test]
fn winner_diag_1(){
	let s = State::from("NNXNXNXNN");
	assert_eq!(1, get_winner_diag(&hash_state(&s)));
}
/*
#[test]
fn winner_all_none(){
	let s = State::from("NNNNNNNNN");
	assert_eq!('N', get_winner(&s));
}

#[test]
fn winner_all_diag_0(){
	let s = State::from("XNNNXNNNX");
	assert_eq!('X', get_winner(&s));
}

#[test]
fn hash_state_0(){
	let s = State::from("NNNNNNNNN");
	assert_eq!(0, hash_state(&s));
}

#[test]
fn hash_state_891(){
	let s = State::from("NNNNONXNN");
	assert_eq!(891, hash_state(&s));
}

#[test]
fn recover_state_0(){
	let s = State::from("NNNNNNNNN");
	assert_eq!(generate_state(0), s);
}

#[test]
fn recover_state_891(){
	let s = State::from("NNNNONXNN");
	assert_eq!(generate_state(891), s);
}

#[test]
fn recover_state_11826(){
	let s = State::from("NNNNONXOX");
	assert_eq!(generate_state(11826), s);
}
*/
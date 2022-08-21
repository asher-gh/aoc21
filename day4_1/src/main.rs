#![allow(unused)]
use std::{
	fs::File,
	io::{BufRead, BufReader, Read, Seek},
};

fn main() {
	let input = File::open("inputs/day4dummy").unwrap();

	let mut reader = BufReader::new(input);

	// gen the drawing numbers
	let mut first_line = String::new();

	reader.read_line(&mut first_line).unwrap();

	// reader.seek(std::io::SeekFrom::Start(0));

	let calls: Vec<u8> = first_line
		.trim()
		.split(",")
		.map(|x| x.parse::<u8>().unwrap())
		.collect();

	dbg!(calls);
}

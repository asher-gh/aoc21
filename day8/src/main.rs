fn main() {
	// for each display the connections are same for inputs and outputs. All I need to
	// do is to get the correct mapping from input and use that for decoding the output
	//
    // basically I have to create a 7 char key string and have mappings for different combinations
    // let's say we create the key 
    //
    //  1(2) -> 7(3) -> 9(6) -> 6(6) -> 2(5)
    // [0, 1, 2, 3, 4, 5, 6]
    // [d, _, _, _, g, _, c]
    //
    // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab 
    //
    //  ddd
    // e   a
    // e   a
    //  fff
    // g   b
    // g   b
    //  ccc
    //
	let input: String = include_str!("../../inputs/day8")
		.trim()
		.lines()
		.map(|x| x.split('|').collect::<Vec<&str>>()[1])
		.collect::<Vec<&str>>()
		.join(" ");

	let counts = input
		.trim()
		.split(" ")
		.filter(|x| match x.len() {
			2 | 3 | 4 | 7 => {
				println!("{x}");
				true
			}
			_ => false,
		})
		.collect::<Vec<&str>>()
		.len();

	dbg!(counts);
	// println!("Hello, world!");
}

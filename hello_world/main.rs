fn main(){
	println!("Hello World !!");

	let mut spaces = "   ";

	println!(spaces);

	let spaces = spaces.len();

	println!(spaces);

	spaces: String = match spaces.parse() {
		Ok(String) => String
		Err(_) => continue;
	}
	spaces = "spaces";

	println!(spaces);


}

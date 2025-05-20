pub fn main(){
	let tokenizer:TokenDict=BufReader::new(File::open("words.txt").unwrap()).lines().filter_map(Result::ok).collect();
	let tokens:Vec<u32>=tokenizer.tokenize_str("text to tokenize here").collect();
	let detokens:String=tokenizer.detokenize_str(&tokens).collect();

	print!("[");
	for id in tokens.iter().take(tokens.len().saturating_sub(1)){print!("{id}, ")}
	if let Some(id)=tokens.last(){print!("{id}")}
	println!("]");
	println!("{detokens}");
}

use {
	std::{
		fs::File,io::{BufRead,BufReader}
	},
	token_dict::{TokenDict}
};

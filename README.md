# token-dict
basic dictionary based tokenization

for example:
```rust
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
```
possible output (depends what's in words.txt):
```
[403933, 32, 410301, 32, 410782, 32, 172340]
text to tokenize here
```

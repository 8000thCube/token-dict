# token-dict
basic dictionary based tokenization

for example:
```rust
pub fn main(){
	let tokenizer:TokenDict=BufReader::new(File::open("words.txt").unwrap()).lines().filter_map(Result::ok).collect();
	let tokens:Vec<u32>=tokenizer.tokenize_str("some text to tokenize").collect();
	let detokens:String=tokenizer.detokenize_str(&tokens).collect();

	print!("[");
	for id in tokens.iter().take(tokens.len().saturating_sub(1)){print!("{id}, ")}
	if let Some(id)=tokens.last(){print!("{id}")}
	println!("]");
	println!("\"{detokens}\"");
}
use {
	std::{
		fs::File,io::{BufRead,BufReader}
	},
	token_dict::{TokenDict}
};
```
possible output (id sequence depends what's in words.txt):
```
[375018, 32, 403933, 32, 410301, 32, 410782]
"some text to tokenize"
```
this tokenizer finds the next token based on whether it's a prefix of the remaining text, so it doesn't need to split on word boundaries first
```rust
pub fn main(){
	let tokenizer:TokenDict=BufReader::new(File::open("words.txt").unwrap()).lines().filter_map(Result::ok).collect();
	let tokens:Vec<u32>=tokenizer.tokenize_str("スペースは不要です").collect();
	let detokens:String=tokenizer.detokenize_str(&tokens).collect();

	print!("[");
	for id in tokens.iter().take(tokens.len().saturating_sub(1)){print!("{id}, ")}
	if let Some(id)=tokens.last(){print!("{id}")}
	println!("]");
	println!("\"{detokens}\"");
}
use {
	std::{
		fs::File,io::{BufRead,BufReader}
	},
	token_dict::{TokenDict}
};
```
possible output (id sequence depends what's in words.txt):
```
[470364, 467937, 471716, 467952]
"スペースは不要です"
```

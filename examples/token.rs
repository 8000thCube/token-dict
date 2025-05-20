pub fn main(){
	let tokenizer:TokenDict=["there","are","tokens","yay"].into_iter().collect();
	let tokens:Vec<u32>=tokenizer.tokenize("there are tokens! yay".as_bytes().iter().map(|&b|b)).collect();
	let detokens:Vec<u8>=tokenizer.detokenize(tokens.clone()).collect();
	//dbg!(tokenizer);
	dbg!(tokens);
	dbg!(String::from_utf8_lossy(&detokens));

	let iter=UTF8CharIter::from("correct string".as_bytes());
	let c:String=iter.filter_map(|c|c.ok()).collect();
	println!("{c}");

	let iter=UTF8CharIter::from("incorrect string".bytes().chain([255,255]).chain(" incorrectness removed".bytes()));
	let c:String=iter.filter_map(|c|c.ok()).collect();
	println!("{c}");

	let iter=UTF8CharIter::from("正しくない文字列".bytes().chain([128,255,0x20,255,0b10000000]).chain(" 不正確さは削除された".bytes()));
	let c:String=iter.filter_map(|c|c.ok()).collect();
	println!("{c}");

}
use token_dict::{UTF8CharIter,TokenDict};

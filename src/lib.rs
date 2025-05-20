const SINGLE:[u8;256]=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255];
impl Default for TokenDict{
	fn default()->Self{
		let x:[Vec<u8>;0]=[];
		x.into_iter().collect()
	}
}
impl DoubleEndedIterator for TokenIntoIter{
	fn next_back(&mut self)->Option<Self::Item>{self.range.next_back().map(|n|self.token[n])}
	fn nth_back(&mut self,n:usize)->Option<Self::Item>{self.range.nth_back(n).map(|n|self.token[n])}
	fn rfold<B,F:FnMut(B,Self::Item)->B>(self,init:B,f:F)->B{self.token.data()[self.range].iter().copied().rfold(init,f)}
}
impl ExactSizeIterator for TokenIntoIter{
	fn len(&self)->usize{self.range.len()}
}
impl Index<u32> for TokenDict{//TODO semantically different implementations for u32 and usize might be confusing
	fn index(&self,ix:u32)->&Self::Output{
		let ix=ix as usize;
		if ix<256{&SINGLE[ix..ix+1]}else{&self.tokens[ix-256][..]}
	}
	type Output=[u8];
}
impl Index<usize> for TokenDict{
	fn index(&self,ix:usize)->&Self::Output{&self.tokens[ix][..]}
	type Output=[u8];
}
impl Index<usize> for Token{
	fn index(&self,ix:usize)->&Self::Output{&self.data()[ix]}
	type Output=u8;
}
impl IntoIterator for Token{
	fn into_iter(self)->Self::IntoIter{
		TokenIntoIter{range:0..self.data().len(),token:self}
	}
	type IntoIter=TokenIntoIter;
	type Item=u8;
}
impl Iterator for TokenIntoIter{
	fn fold<B,F:FnMut(B,Self::Item)->B>(self,init:B,f:F)->B{self.token.data()[self.range].iter().copied().fold(init,f)}
	fn next(&mut self)->Option<Self::Item>{self.range.next().map(|n|self.token[n])}
	fn nth(&mut self,n:usize)->Option<Self::Item>{self.range.nth(n).map(|n|self.token[n])}
	fn size_hint(&self)->(usize,Option<usize>){self.range.size_hint()}
	type Item=u8;
}
impl Token{
	/// returns the token bytes
	pub fn data(&self)->&[u8]{
		let id=self.id as usize;
		let token=&self.token;

		if id<256{&SINGLE[id..id+1]}else{token.as_deref().unwrap()}
	}
	/// returns the token id
	pub fn id(&self)->u32{self.id}
}
impl TokenDict{
	/// decodes the tokens into bytes
	pub fn detokenize<I:IntoIterator>(&self,tokens:I)->DetokenIter<I::IntoIter> where I::Item:Val<u32>{
		DetokenIter{inner:tokens.into_iter(),maxtokenlen:self.maxtokenlen,position:1,tokenid:0,tokens:self.tokens.clone()}
	}
	/// returns an interator over the tokens that does not include the implicit 256 bytes
	pub fn dict_iter(&self)->impl '_+DoubleEndedIterator<Item=Token>+ExactSizeIterator<Item=Token>{(256..self.tokens.len()+256).map(|n|self.token(n as u32))}
	/// returns the length of the token list that does not include the implicit 256 bytes
	pub fn dict_len(&self)->usize{self.tokens.len()}
	/// returns an interator over the tokens that includes the implicit 256 bytes
	pub fn full_iter(&self)->impl '_+DoubleEndedIterator<Item=Token>+ExactSizeIterator<Item=Token>{(0..self.tokens.len()+256).map(|n|self.token(n as u32))}
	/// returns the length of the token list that includes the implicit 256 bytes
	pub fn full_len(&self)->usize{self.tokens.len()+256}
	/// returns the count of ids that could be output by the tokenizer. This incluces the single bytes than implicitly form the lower 256 ids
	pub fn id_count(&self)->usize{self.tokens.len()+256}
	/// returns an iterator over the tokens
	pub fn iter(&self)->impl DoubleEndedIterator<Item=&[u8]>+ExactSizeIterator<Item=&[u8]>{self.tokens.iter().map(|t|&t[..])}
	/// returns the number of tokens in the dictionary, not including the single bytes which implicitly form the lower 256 ids. Use id_count to include these
	pub fn len(&self)->usize{self.tokens.len()}
	/// converts the string to a token vec
	pub fn string_to_tokens<S:AsRef<str>>(&self,input:S)->Vec<u32>{self.tokenize(input.as_ref().as_bytes()).collect()}
	/// returns the token with the id
	pub fn token(&self,id:u32)->Token{
		let token=if id<256{None}else{Some(self.tokens[id as usize-256].clone())};
		Token{id,token}
	}
	/// converts the bytes to tokens
	pub fn tokenize<I:IntoIterator>(&self,bytes:I)->TokenIter<I::IntoIter> where I::Item:Val<u8>{
		//TokenIter{ids:self.ids.clone(),inner:bytes.into_iter(),maxtokenlen:self.maxtokenlen,previous:None}
		TokenIter{ids:self.ids.clone(),inner:bytes.into_iter(),state:VecDeque::with_capacity(self.maxtokenlen)}
	}
	/// converts the token vec to string
	pub fn tokens_to_string<V:AsRef<[u32]>>(&self,input:V)->String{String::from_utf8_lossy(&self.detokenize(input.as_ref()).collect::<Vec<u8>>()).to_string()}
}
impl<I:AsRef<[u8]>> FromIterator<I> for TokenDict{
	fn from_iter<J:IntoIterator<Item=I>>(iter:J)->Self{
		let mut maxtokenlen=1;
		let mut ids=Trie::new();
		let tokens:Vec<Arc<[u8]>>=iter.into_iter().filter(|t|t.as_ref().len()>1).enumerate().map(|(n,t)|{
			let id=u32::try_from(n+256).unwrap();
			let token:Arc<[u8]>=Arc::from(t.as_ref());
			ids.insert(token.iter().copied(),id);
			maxtokenlen=maxtokenlen.max(token.len());
			token
		}).collect();

		let (ids,tokens)=(Arc::new(ids),Arc::new(tokens));
		Self{ids,maxtokenlen,tokens}
	}
}
impl<I:IntoIterator> From<I> for UTF8CharIter<I::IntoIter> where I::Item:Val<u8>{
	fn from(value:I)->Self{
		Self{inner:value.into_iter()}
	}
}
impl<I:Iterator> DetokenIter<I> where I::Item:Val<u32>{
	/// converts into the innter value
	pub fn into_inner(self)->I{self.inner}
}
impl<I:Iterator> Iterator for DetokenIter<I> where I::Item:Val<u32>{
	fn fold<B,F:FnMut(B,Self::Item)->B>(self,init:B,mut f:F)->B{
		self.inner.map(Val::val).fold(init,|acc,tokenid|if tokenid<256{f(acc,tokenid as u8)}else{self.tokens[tokenid as usize].iter().fold(acc,|acc,&b|f(acc,b))})
	}
	fn next(&mut self)->Option<u8>{
		let (inner,position)=(&mut self.inner,&mut self.position);
		let tokenid=&mut self.tokenid;
		let tokens=&self.tokens;

		if let Some(b)=if *tokenid<256{(*position==0).then_some(*tokenid as u8)}else{tokens[*tokenid as usize-256].get(*position).map(|&b|b)}{
			*position+=1;
			b
		}else{
			*position=1;
			*tokenid=inner.map(Val::val).next()?;
			if *tokenid<256{*tokenid as u8}else{tokens[*tokenid as usize-256][0]}//TODO fix in case a token len becomes 0
		}.into()
	}
	fn size_hint(&self)->(usize,Option<usize>){
		let (lowertokens,uppertokens)=self.inner.size_hint();
		let maxtoken=self.maxtokenlen;

		(lowertokens,uppertokens.map(|h|h*maxtoken))
	}
	type Item=u8;
}
impl<I:Iterator> Iterator for TokenIter<I> where I::Item:Val<u8>{
	fn next(&mut self)->Option<u32>{
		let (inner,state)=(&mut self.inner,&mut self.state);
		let ids=&self.ids;
		state.extend(inner.map(Val::val).take(state.capacity()-state.len()));
		if state.len()==0{return None}

		let (tokenlen,&tokenid)=if let Some(t)=ids.find_longest_prefix_len(state.iter().copied()).filter(|(tokenlen,_tokenid)|*tokenlen>0){t}else{return Some(state.pop_front().unwrap() as u32)};
		for _ in 0..tokenlen{
			state.pop_front();
		}
		return Some(tokenid)
	}
	fn size_hint(&self)->(usize,Option<usize>){
		let (lowerbytes,upperbytes)=self.inner.size_hint();
		let maxtoken=self.state.capacity();
		let statelen=self.state.len();

		((lowerbytes+statelen).div_ceil(maxtoken),upperbytes.map(|b|b+statelen))
	}
	type Item=u32;
}
impl<I:Iterator> Iterator for UTF8CharIter<I> where I::Item:Val<u8>{
	fn next(&mut self)->Option<Result<char,[u8;4]>>{
		let inner=&mut self.inner;

		let firstbyte=inner.next()?.val();
		let mut bytes=[firstbyte,0,0,0];
		let (charlen,value)=if firstbyte&0b10000000==0b00000000{(1,firstbyte)}
		else if firstbyte&0b11000000==0b10000000{return Some(Err(bytes))}
		else if firstbyte&0b11100000==0b11000000{(2,firstbyte&0b00011111)}
		else if firstbyte&0b11110000==0b11100000{(3,firstbyte&0b00001111)}
		else if firstbyte&0b11111000==0b11110000{(3,firstbyte&0b00000111)}
		else									   {return Some(Err(bytes))};
		let mut value=value as u32;
		for n in 1..charlen{
			let nextbyte=if let Some(b)=inner.next(){b.val()}else{return Some(Err(bytes))};
			bytes[n]=nextbyte;
			if nextbyte&0b11000000==0b10000000{value=(value<<6)+((nextbyte&0b00111111) as u32)}else{return Some(Err(bytes))}
		}
		char::from_u32(value).map(|c|Ok(c)).or(Some(Err(bytes)))
	}
	fn size_hint(&self)->(usize,Option<usize>){
		let (lowerbytes,upperbytes)=self.inner.size_hint();

		(lowerbytes/4,upperbytes)
	}
	type Item=Result<char,[u8;4]>;
}
impl<I:Iterator> TokenIter<I> where I::Item:Val<u8>{
	/// converts into the inner value
	pub fn into_inner(self)->I{self.inner}
}
impl<I:Iterator> UTF8CharIter<I> where I::Item:Val<u8>{
	/// converts into the inner value
	pub fn into_inner(self)->I{self.inner}
}
impl<T:Copy> Val<T> for &T{
	fn val(self)->T{*self}
}
impl<T> Val<T> for T{
	fn val(self)->T{self}
}
#[derive(Clone,Debug)]
/// a simple dictionary based detokenizer
pub struct DetokenIter<I:Iterator> where I::Item:Val<u32>{inner:I,maxtokenlen:usize,position:usize,tokenid:u32,tokens:Arc<Vec<Arc<[u8]>>>}
#[derive(Clone,Debug)]
/// a reference counted structure with a token and its id
pub struct Token{id:u32,token:Option<Arc<[u8]>>}
#[derive(Clone,Debug)]
/// a simple dictionary based tokenizer dictionary where single bytes implicitly form the lower 256 ids. It's reference counted and cheap to clone
pub struct TokenDict{ids:Arc<Trie<u8,u32>>,maxtokenlen:usize,tokens:Arc<Vec<Arc<[u8]>>>}
#[derive(Clone,Debug)]
/// intoiterator for Token
pub struct TokenIntoIter{range:Range<usize>,token:Token}
#[derive(Clone,Debug)]
/// a simple dictionary based tokenizer iterator
pub struct TokenIter<I:Iterator> where I::Item:Val<u8>{ids:Arc<Trie<u8,u32>>,inner:I,state:VecDeque<u8>}
#[derive(Clone,Debug)]
/// iterator for live converting utf8 to chars, returning errors for all bytes that aren't part of a valid character. useful for lazily detokenizing into a string
pub struct UTF8CharIter<I:Iterator> where I::Item:Val<u8>{inner:I}
/// because primitive doesn't implement AsRef for some godforsaken reason
pub trait Val<T>{
	/// returns the item as a value
	fn val(self)->T;
}
use {
	ptrie::Trie,
	std::{
		collections::VecDeque,ops::{Index,Range},sync::Arc,
	},
};

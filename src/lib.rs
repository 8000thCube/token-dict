const SINGLE_BYTES:&[u8;256]=&[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255];
const SINGLE_TOKENS:&[Token;256]=&[Token::single(0),Token::single(1),Token::single(2),Token::single(3),Token::single(4),Token::single(5),Token::single(6),Token::single(7),Token::single(8),Token::single(9),Token::single(10),Token::single(11),Token::single(12),Token::single(13),Token::single(14),Token::single(15),Token::single(16),Token::single(17),Token::single(18),Token::single(19),Token::single(20),Token::single(21),Token::single(22),Token::single(23),Token::single(24),Token::single(25),Token::single(26),Token::single(27),Token::single(28),Token::single(29),Token::single(30),Token::single(31),Token::single(32),Token::single(33),Token::single(34),Token::single(35),Token::single(36),Token::single(37),Token::single(38),Token::single(39),Token::single(40),Token::single(41),Token::single(42),Token::single(43),Token::single(44),Token::single(45),Token::single(46),Token::single(47),Token::single(48),Token::single(49),Token::single(50),Token::single(51),Token::single(52),Token::single(53),Token::single(54),Token::single(55),Token::single(56),Token::single(57),Token::single(58),Token::single(59),Token::single(60),Token::single(61),Token::single(62),Token::single(63),Token::single(64),Token::single(65),Token::single(66),Token::single(67),Token::single(68),Token::single(69),Token::single(70),Token::single(71),Token::single(72),Token::single(73),Token::single(74),Token::single(75),Token::single(76),Token::single(77),Token::single(78),Token::single(79),Token::single(80),Token::single(81),Token::single(82),Token::single(83),Token::single(84),Token::single(85),Token::single(86),Token::single(87),Token::single(88),Token::single(89),Token::single(90),Token::single(91),Token::single(92),Token::single(93),Token::single(94),Token::single(95),Token::single(96),Token::single(97),Token::single(98),Token::single(99),Token::single(100),Token::single(101),Token::single(102),Token::single(103),Token::single(104),Token::single(105),Token::single(106),Token::single(107),Token::single(108),Token::single(109),Token::single(110),Token::single(111),Token::single(112),Token::single(113),Token::single(114),Token::single(115),Token::single(116),Token::single(117),Token::single(118),Token::single(119),Token::single(120),Token::single(121),Token::single(122),Token::single(123),Token::single(124),Token::single(125),Token::single(126),Token::single(127),Token::single(128),Token::single(129),Token::single(130),Token::single(131),Token::single(132),Token::single(133),Token::single(134),Token::single(135),Token::single(136),Token::single(137),Token::single(138),Token::single(139),Token::single(140),Token::single(141),Token::single(142),Token::single(143),Token::single(144),Token::single(145),Token::single(146),Token::single(147),Token::single(148),Token::single(149),Token::single(150),Token::single(151),Token::single(152),Token::single(153),Token::single(154),Token::single(155),Token::single(156),Token::single(157),Token::single(158),Token::single(159),Token::single(160),Token::single(161),Token::single(162),Token::single(163),Token::single(164),Token::single(165),Token::single(166),Token::single(167),Token::single(168),Token::single(169),Token::single(170),Token::single(171),Token::single(172),Token::single(173),Token::single(174),Token::single(175),Token::single(176),Token::single(177),Token::single(178),Token::single(179),Token::single(180),Token::single(181),Token::single(182),Token::single(183),Token::single(184),Token::single(185),Token::single(186),Token::single(187),Token::single(188),Token::single(189),Token::single(190),Token::single(191),Token::single(192),Token::single(193),Token::single(194),Token::single(195),Token::single(196),Token::single(197),Token::single(198),Token::single(199),Token::single(200),Token::single(201),Token::single(202),Token::single(203),Token::single(204),Token::single(205),Token::single(206),Token::single(207),Token::single(208),Token::single(209),Token::single(210),Token::single(211),Token::single(212),Token::single(213),Token::single(214),Token::single(215),Token::single(216),Token::single(217),Token::single(218),Token::single(219),Token::single(220),Token::single(221),Token::single(222),Token::single(223),Token::single(224),Token::single(225),Token::single(226),Token::single(227),Token::single(228),Token::single(229),Token::single(230),Token::single(231),Token::single(232),Token::single(233),Token::single(234),Token::single(235),Token::single(236),Token::single(237),Token::single(238),Token::single(239),Token::single(240),Token::single(241),Token::single(242),Token::single(243),Token::single(244),Token::single(245),Token::single(246),Token::single(247),Token::single(248),Token::single(249),Token::single(250),Token::single(251),Token::single(252),Token::single(253),Token::single(254),Token::single(255)];
impl AsMut<Self> for Token{
	fn as_mut(&mut self)->&mut Self{self}
}
impl AsMut<Self> for TokenDict{
	fn as_mut(&mut self)->&mut Self{self}
}
impl AsRef<[u8]> for Token{
	fn as_ref(&self)->&[u8]{self.data()}
}
impl AsRef<Self> for Token{
	fn as_ref(&self)->&Self{self}
}
impl AsRef<Self> for TokenDict{
	fn as_ref(&self)->&Self{self}
}
impl Default for TokenDict{
	fn default()->Self{
		let x:[Vec<u8>;0]=[];
		x.into_iter().collect()
	}
}
impl Deref for Token{
	fn deref(&self)->&Self::Target{self.data()}
	type Target=[u8];
}
impl DoubleEndedIterator for DictIntoIter{
	fn next_back(&mut self)->Option<Self::Item>{
		self.range.next_back().map(|n|if n<256{SINGLE_TOKENS[n].clone()}else{self.tokens[n-256].clone()})
	}
	fn nth_back(&mut self,n:usize)->Option<Self::Item>{
		self.range.nth_back(n).map(|n|if n<256{SINGLE_TOKENS[n].clone()}else{self.tokens[n-256].clone()})
	}
	fn rfold<B,F:FnMut(B,Self::Item)->B>(self,mut init:B,mut f:F)->B{
		let (range,tokens)=(self.range,self.tokens);
		let (start,stop)=(range.start,range.end);
		init=tokens[start.saturating_sub(256)..stop.saturating_sub(256)].iter().cloned().rfold(init,&mut f);
		SINGLE_TOKENS[start.min(256)..stop.min(256)].iter().cloned().rfold(init,f)
	}
}
impl DoubleEndedIterator for TokenIntoIter{
	fn next_back(&mut self)->Option<Self::Item>{self.range.next_back().map(|n|self.token[n])}
	fn nth_back(&mut self,n:usize)->Option<Self::Item>{self.range.nth_back(n).map(|n|self.token[n])}
	fn rfold<B,F:FnMut(B,Self::Item)->B>(self,init:B,f:F)->B{self.token.data()[self.range].iter().copied().rfold(init,f)}
}
impl ExactSizeIterator for DictIntoIter{
	fn len(&self)->usize{self.range.len()}
}
impl ExactSizeIterator for TokenIntoIter{
	fn len(&self)->usize{self.range.len()}
}
impl Index<u32> for TokenDict{
	fn index(&self,ix:u32)->&Self::Output{
		let ix=ix as usize;
		if ix<256{&SINGLE_BYTES[ix..ix+1]}else{&self.tokens[ix-256]}
	}
	type Output=[u8];
}
impl Index<usize> for TokenDict{
	fn index(&self,ix:usize)->&Self::Output{
		if ix<256{&SINGLE_TOKENS[ix]}else{&self.tokens[ix-256]}
	}
	type Output=Token;
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
impl IntoIterator for TokenDict{
	fn into_iter(self)->Self::IntoIter{
		DictIntoIter{range:0..self.len(),tokens:self.tokens}
	}
	type IntoIter=DictIntoIter;
	type Item=Token;
}
impl Iterator for DictIntoIter{
	fn count(self)->usize{self.range.count()}
	fn fold<B,F:FnMut(B,Self::Item)->B>(self,mut init:B,mut f:F)->B{
		let (range,tokens)=(self.range,self.tokens);
		let (start,stop)=(range.start,range.end);
		init=SINGLE_TOKENS[start.min(256)..stop.min(256)].iter().cloned().rfold(init,&mut f);
		tokens[start.saturating_sub(256)..stop.saturating_sub(256)].iter().cloned().rfold(init,f)
	}
	fn last(mut self)->Option<Self::Item>{self.next_back()}
	fn next(&mut self)->Option<Self::Item>{
		self.range.next().map(|n|if n<256{SINGLE_TOKENS[n].clone()}else{self.tokens[n-256].clone()})
	}
	fn nth(&mut self,n:usize)->Option<Self::Item>{
		self.range.nth(n).map(|n|if n<256{SINGLE_TOKENS[n].clone()}else{self.tokens[n-256].clone()})
	}
	fn size_hint(&self)->(usize,Option<usize>){self.range.size_hint()}
	type Item=Token;
}
impl Iterator for TokenIntoIter{
	fn count(self)->usize{self.range.count()}
	fn fold<B,F:FnMut(B,Self::Item)->B>(self,init:B,f:F)->B{self.token.data()[self.range].iter().copied().fold(init,f)}
	fn last(mut self)->Option<Self::Item>{self.next_back()}
	fn next(&mut self)->Option<Self::Item>{self.range.next().map(|n|self.token[n])}
	fn nth(&mut self,n:usize)->Option<Self::Item>{self.range.nth(n).map(|n|self.token[n])}
	fn size_hint(&self)->(usize,Option<usize>){self.range.size_hint()}
	type Item=u8;
}
impl Token{
	/// creates a token from a single byte
	const fn single(n:u8)->Token{
		Self{id:n as u32,token:None}
	}
	/// returns the token bytes
	pub fn data(&self)->&[u8]{
		let id=self.id as usize;
		let token=&self.token;

		if id<256{&SINGLE_BYTES[id..id+1]}else{token.as_deref().unwrap()}
	}
	/// returns the token id
	pub fn id(&self)->u32{self.id}
}
impl TokenDict{
	/// decodes the tokens into bytes
	pub fn detokenize<I:IntoIterator>(&self,tokens:I)->Detokenization<I::IntoIter> where I::Item:Val<u32>{
		Detokenization{inner:tokens.into_iter(),maxtokenlen:self.maxtokenlen,position:1,tokenid:0,tokens:self.tokens.clone()}
	}
	/// creates an iterator over tokens
	pub fn detoken_iter<I:IntoIterator>(&self,tokens:I)->impl Iterator<Item=Token> where I::Item:Val<u32>{
		let tokenizer=self.clone();
		tokens.into_iter().map(move|id|tokenizer[id.val() as usize].clone())
	}
	/// decodes the tokens into chars, replacing invalid unicode with replacement character
	pub fn detokenize_str<I:IntoIterator>(&self,tokens:I)->impl Iterator<Item=char> where I::Item:Val<u32>{
		UTF8CharIter::from(self.detokenize(tokens)).map(|r|if let Ok(c)=r{c}else{char::REPLACEMENT_CHARACTER})
	}
	/// returns an interator over the possible tokens generated by this tokenizer
	pub fn iter(&self)->DictIter<'_>{
		DictIter{range:0..self.len(),tokens:&self.tokens}
	}
	/// returns the number of possible token ids generated by this tokenizer
	pub fn len(&self)->usize{self.tokens.len()+256}
	/// converts the string to a token vec
	pub fn string_to_tokens<S:?Sized+AsRef<str>>(&self,input:&S)->Vec<u32>{self.tokenize(input.as_ref().as_bytes()).collect()}
	/// creates an iterator over tokens
	pub fn token_iter<I:IntoIterator>(&self,bytes:I)->impl Iterator<Item=Token> where I::Item:Val<u8>{
		let tokenizer=self.clone();
		self.tokenize(bytes).map(move|id|tokenizer[id as usize].clone())
	}
	/// converts the bytes to tokens
	pub fn tokenize<I:IntoIterator>(&self,bytes:I)->Tokenization<I::IntoIter> where I::Item:Val<u8>{
		Tokenization{ids:self.ids.clone(),inner:bytes.into_iter(),state:VecDeque::with_capacity(self.maxtokenlen)}
	}
	/// converts the string to tokens
	pub fn tokenize_str<'a,S:?Sized+AsRef<str>>(&self,input:&'a S)->Tokenization<SliceIter<'a,u8>>{self.tokenize(input.as_ref().as_bytes())}
	/// converts the string to tokens
	pub fn tokenize_string(&self,input:String)->Tokenization<VecIntoIter<u8>>{self.tokenize(Vec::from(input))}
	/// converts the token vec to string
	pub fn tokens_to_string<V:?Sized+AsRef<[u32]>>(&self,input:&V)->String{String::from_utf8_lossy(&self.detokenize(input.as_ref()).collect::<Vec<u8>>()).to_string()}
}
impl<'a> DoubleEndedIterator for DictIter<'a>{
	fn next_back(&mut self)->Option<Self::Item>{
		self.range.next_back().map(|n|if n<256{&SINGLE_TOKENS[n]}else{&self.tokens[n-256]})
	}
	fn nth_back(&mut self,n:usize)->Option<Self::Item>{
		self.range.nth_back(n).map(|n|if n<256{&SINGLE_TOKENS[n]}else{&self.tokens[n-256]})
	}
	fn rfold<B,F:FnMut(B,Self::Item)->B>(self,mut init:B,mut f:F)->B{
		let (range,tokens)=(self.range,self.tokens);
		let (start,stop)=(range.start,range.end);
		init=tokens[start.saturating_sub(256)..stop.saturating_sub(256)].iter().rfold(init,&mut f);
		SINGLE_TOKENS[start.min(256)..stop.min(256)].iter().rfold(init,f)
	}
}
impl<'a> ExactSizeIterator for DictIter<'a>{
	fn len(&self)->usize{self.range.len()}
}
impl<'a> Iterator for DictIter<'a>{
	fn count(self)->usize{self.range.count()}
	fn fold<B,F:FnMut(B,Self::Item)->B>(self,mut init:B,mut f:F)->B{
		let (range,tokens)=(self.range,self.tokens);
		let (start,stop)=(range.start,range.end);
		init=SINGLE_TOKENS[start.min(256)..stop.min(256)].iter().rfold(init,&mut f);
		tokens[start.saturating_sub(256)..stop.saturating_sub(256)].iter().rfold(init,f)
	}
	fn last(mut self)->Option<Self::Item>{self.next_back()}
	fn next(&mut self)->Option<Self::Item>{
		self.range.next().map(|n|if n<256{&SINGLE_TOKENS[n]}else{&self.tokens[n-256]})
	}
	fn nth(&mut self,n:usize)->Option<Self::Item>{
		self.range.nth(n).map(|n|if n<256{&SINGLE_TOKENS[n]}else{&self.tokens[n-256]})
	}
	fn size_hint(&self)->(usize,Option<usize>){self.range.size_hint()}
	type Item=&'a Token;
}
impl<A:AsRef<[u8]>> Extend<A> for TokenDict{
	fn extend<I:IntoIterator<Item=A>>(&mut self,iter:I){
		let (ids,tokens)=(Arc::make_mut(&mut self.ids),Arc::make_mut(&mut self.tokens));
		let maxtokenlen=&mut self.maxtokenlen;

		iter.into_iter().for_each(|a|{
			let id=u32::try_from(tokens.len()+256).unwrap();
			let token:Arc<[u8]>=Arc::from(a.as_ref());
			ids.insert(token.iter().copied(),id);
			*maxtokenlen=(*maxtokenlen).max(token.len());
			tokens.push(Token{id,token:Some(token)})
		});
	}
}
impl<A:AsRef<[u8]>> FromIterator<A> for TokenDict{
	fn from_iter<I:IntoIterator<Item=A>>(iter:I)->Self{
		let mut maxtokenlen=1;
		let mut ids=Trie::new();
		let tokens:Vec<Token>=iter.into_iter().filter(|t|t.as_ref().len()>1).enumerate().map(|(n,t)|{
			let id=u32::try_from(n+256).unwrap();
			let token:Arc<[u8]>=Arc::from(t.as_ref());
			ids.insert(token.iter().copied(),id);
			maxtokenlen=maxtokenlen.max(token.len());
			Token{id,token:Some(token)}
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
impl<I:Iterator> Detokenization<I> where I::Item:Val<u32>{
	/// converts into the innter value
	pub fn into_inner(self)->I{self.inner}
}
impl<I:Iterator> Iterator for Detokenization<I> where I::Item:Val<u32>{
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
			if *tokenid<256{*tokenid as u8}else{tokens[*tokenid as usize-256][0]}
		}.into()
	}
	fn size_hint(&self)->(usize,Option<usize>){
		let (lowertokens,uppertokens)=self.inner.size_hint();
		let maxtoken=self.maxtokenlen;

		(lowertokens,uppertokens.map(|h|h*maxtoken))
	}
	type Item=u8;
}
impl<I:Iterator> Iterator for Tokenization<I> where I::Item:Val<u8>{
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
impl<I:Iterator> Tokenization<I> where I::Item:Val<u8>{
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
#[cfg(test)]
mod tests{
	#[test]
	fn tokenizer_iter(){
		let tokenizer:TokenDict=["aa","bb","cc"].into_iter().collect();
		let t2:TokenDict=tokenizer.iter().collect();
		assert_eq!(tokenizer.tokenize_str("ccaabb").collect::<Vec<_>>(),t2.tokenize_str("ccaabb").collect::<Vec<_>>());

	}
	#[test]
	fn bytes_only(){
		let teststring="oishsoghohhduihahdufghud";
		let tokenizer=TokenDict::default();
		let tokens:Vec<u32>=tokenizer.tokenize_str(teststring).collect();
		let detokens:Vec<u8>=tokenizer.detokenize(tokens).collect();

		assert_eq!(detokens.as_slice(),teststring.as_bytes());
	}
	#[test]
	fn utf8_test_1(){
		let iter=UTF8CharIter::from("correct string".as_bytes());
		let c:String=iter.filter_map(|c|c.ok()).collect();
		assert_eq!(c,"correct string");
		let iter=UTF8CharIter::from("incorrect string".bytes().chain([255,255]).chain(" incorrectness removed".bytes()));
		let c:String=iter.filter_map(|c|c.ok()).collect();
		assert_eq!(c,"incorrect string incorrectness removed");

		let iter=UTF8CharIter::from("正しくない文字列".bytes().chain([128,255,0x20,255,0b10000000]).chain(" 不正確さは削除された".bytes()));
		let c:String=iter.filter_map(|c|c.ok()).collect();
		assert_eq!(c,"正しくない文字列  不正確さは削除された");
	}
	#[test]
	fn there_are_tokens_yay(){
		let teststring="there are tokens! yay";
		let tokenizer:TokenDict=["there","are","tokens","yay"].into_iter().collect();
		let tokens:Vec<u32>=tokenizer.tokenize(teststring.bytes()).collect();
		let detokens:Vec<u8>=tokenizer.detokenize(&tokens).collect();

		assert_eq!(tokens.len(),8);
		assert_eq!(detokens.as_slice(),teststring.as_bytes());
	}
	#[test]
	fn test_default_token_dict_detokenize_empty() {
		let dict = TokenDict::default();
		let inp:Vec<u32>=vec![];
		let out: Vec<u8> = dict.detokenize(inp).collect();
		assert!(out.is_empty(), "Detokenizing an empty input should yield no bytes");
	}

	#[test]
	fn test_token_into_iter_single_byte() {
		// Use a Token with id < 256 and no custom data
		let token = Token { id: 5, token: None };
		let v: Vec<u8> = token.clone().into_iter().collect();
		// SINGLE_BYTES[5] == 5
		assert_eq!(v, vec![5]);
	}

	#[test]
	fn test_token_into_iter_multi_byte() {
		// Create a custom token of two bytes [10, 20]
		let data:Arc<[u8]> = Arc::from([10u8, 20u8].as_ref());
		let token = Token { id: 256, token: Some(data.clone()) };
		let mut iter = token.clone().into_iter();
		// next should yield first element
		assert_eq!(iter.next(), Some(10));
		// next_back should yield last element
		assert_eq!(iter.next_back(), Some(20));
	}

	#[test]
	fn test_exact_size_iterator_len() {
		// Custom two-byte token
		let data:Arc<[u8]> = Arc::from([30u8, 40u8].as_ref());
		let token = Token { id: 300, token: Some(data.clone()) };
		let iter = token.into_iter();
		assert_eq!(iter.len(), 2, "ExactSizeIterator.len() should reflect number of bytes");
	}

	#[test]
	fn test_utf8_char_iter_valid() {
		// Unicode character '€' (euro sign)
		let bytes = vec![0xE2u8, 0x82u8, 0xACu8];
		let mut iter: UTF8CharIter<_> = bytes.into_iter().into();
		match iter.next() {
			Some(Ok(c)) => assert_eq!(c, '€'),
			other => panic!("Expected Ok('€'), got {:?}", other),
		}
		// No more characters
		assert!(iter.next().is_none());
	}

	#[test]
	fn test_utf8_char_iter_error() {
		// Invalid UTF-8 start byte
		let bytes = vec![0xFFu8];
		let mut iter: UTF8CharIter<_> = bytes.into_iter().into();
		match iter.next() {
			Some(Err(buf)) => assert_eq!(buf[0], 0xFF),
			other => panic!("Expected Err with first byte 0xFF, got {:?}", other),
		}
	}
	use super::*;
}
#[derive(Clone,Debug)]
/// a simple dictionary based detokenizer
pub struct Detokenization<I:Iterator> where I::Item:Val<u32>{inner:I,maxtokenlen:usize,position:usize,tokenid:u32,tokens:Arc<Vec<Token>>}
#[derive(Clone,Debug)]
/// iterator for TokenDict
pub struct DictIter<'a>{range:Range<usize>,tokens:&'a [Token]}
#[derive(Clone,Debug)]
/// intoiterator for TokenDict
pub struct DictIntoIter{range:Range<usize>,tokens:Arc<Vec<Token>>}
#[derive(Clone,Debug,Default)]
/// a reference counted structure with a token and its id
pub struct Token{id:u32,token:Option<Arc<[u8]>>}
#[derive(Clone,Debug)]
/// a simple dictionary based tokenizer dictionary where single bytes implicitly form the lower 256 ids. It's reference counted and cheap to clone
pub struct TokenDict{ids:Arc<Trie<u8,u32>>,maxtokenlen:usize,tokens:Arc<Vec<Token>>}
#[derive(Clone,Debug)]
/// intoiterator for Token
pub struct TokenIntoIter{range:Range<usize>,token:Token}
#[derive(Clone,Debug)]
/// a simple dictionary based tokenizer iterator
pub struct Tokenization<I:Iterator> where I::Item:Val<u8>{ids:Arc<Trie<u8,u32>>,inner:I,state:VecDeque<u8>}
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
		collections::VecDeque,iter::Extend,ops::{Deref,Index,Range},slice::Iter as SliceIter,sync::Arc,vec::IntoIter as VecIntoIter,
	},
};

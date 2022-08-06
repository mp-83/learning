fn callee(s: &mut String) -> &mut String {
	s.push('!');
	s	
}



pub fn pub_caller() -> &'static String {
	let mut s = String::from("marco");
	let res = callee(&mut s);
	res
}

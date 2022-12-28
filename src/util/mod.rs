///Function used to convert the first letter of the string to uppercase, to it can be parsed and displayed correctly
pub fn uppercase_converter(s: &mut String) {
	let mut v: Vec<char> = s.chars().collect();
	v[0] = v[0].to_uppercase().nth(0).unwrap();
	let s2: String = v.into_iter().collect();
	*s = s2.clone();
}

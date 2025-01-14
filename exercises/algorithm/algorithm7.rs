/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		match self.data.pop() {
			Some(t) => {
				self.size -= 1;
				Some(t)
			},
			None => {
				None
			}
		}

	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {  //? peek_mut: similar to peek, but mutable reference
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {     //? take ownership
		IntoIter(self)                      //? type of `self`: Stack<T>
	}
	fn iter(&self) -> Iter<T> {             //? Don't take ownership, immutable
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {   //? Don't take ownership, mutable
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);              //? tuple struct, first element is Stack<T>
impl<T: Clone> Iterator for IntoIter<T> {  //? 为这个tuple struct: IntoIter<T>实现Iterator trait
	type Item = T;             //? take ownership
	fn next(&mut self) -> Option<Self::Item> {  //? next() implementation in Iterator trait
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {       //? struct Iter<T>,  a': 生命周期标识
	stack: Vec<&'a T>,         //? struct Iter<T>的一个成员stack, 类型是Vec<&T>
}
impl<'a, T> Iterator for Iter<'a, T> {   //? 为struct Iter实现Iterator trait
	type Item = &'a T;         //? don't take ownership, immutable reference
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {    //? stack IterMut<T>
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;    //? don't take ownership, mutable reference
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn check_bracket(c: char) -> bool {
	if c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}' {
		return true;
	}
	else {
		return false;
	}
}

fn bracket_match(bracket: &str) -> bool
{	
	let mut s = Stack::<char>::new();
	for c in bracket.chars() {
		match c {
			'{' | '[' | '('  => s.push(c),
			'}' => {
				if s.pop() != Some('{') {
					return false;
				}
			},
			']' => {
				if s.pop() != Some('[') {
					return false;
				}
			},
			')' => {
				if s.pop() != Some('(') {
					return false;
				}
			},
			_ => continue,
		}
	}

	if !s.is_empty() {
		return false;
	} else {
		return true;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}
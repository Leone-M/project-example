#[cfg(test)]
mod tests {
use project_example::math_functions;


	#[test]
	fn test_add() {
		let z = math_functions::add::add_func(&10, &10);
		assert_eq!(z, 20);
	}
	#[test]
	fn test_sub() {
		let z = math_functions::sub::sub_func(&10, &10);
		assert_eq!(z, 0);
	}
	#[test]
	#[should_panic]
	fn test_panic(){
		let z = math_functions::sub::sub_func(&10, &10);
		assert_eq!(z, 20)
	}
	#[test]
	#[should_panic]
	fn test_failure_example(){
		let z = math_functions::add::add_func(&10, &10);
		assert_eq!(z, 0)
	}
}
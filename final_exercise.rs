use std::os;

fn main() {
	if os::args().len() < 2 {
		println("Error: Please provide a number as argument.");
		return;
	}

	let i = from_str::<int>(os::args()[1]).unwrap();
	let mut number = 1;
	loop {
	     if collatz(number) == i {
	     	println!("{:d}", number);
		break;
	     }
	     number = number + 1;
	}
}

fn collatz(N: int) -> int {
	if N == 1 { return 0; }
	match N % 2 {
		0 => { 1 + collatz(N/2) }
		_ => { 1 + collatz(N*3+1) }
	}
}
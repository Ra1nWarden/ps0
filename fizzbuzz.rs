fn main() {
   for i in range(1, 101) {
       if i % 15 == 0 {
       	  println("FizzBuzz");
       }
       else if i % 3 == 0 {
       	  println("Fizz");
       }
       else if i % 5 == 0 {
       	  println("Buzz");
       }
       else {
	  println!("{:d}", i);
       }
   }
}
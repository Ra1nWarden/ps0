fn main() {
   let x = (51, true);
   match x {
   	 (x, true) if x > 20 && x < 26 => { println("(a) is true"); }
	 (x, true) if x <= 20 || x >= 26 => { println("(b) is true"); }
	 (x, _) if x > 40 && x < 49 => { println("(c) is true"); }
	 (_,_) => { println("none of the above is true"); }
   }
}
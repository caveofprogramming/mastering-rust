fn main() {
   let mut values:Vec<i32> = vec![1, 2, 3];

   println!("{values:?}");

   values[1] = 4;
   println!("{values:?}");
   
   for v in &values {
       print!("{v} ");
   }

   println!();

   values.push(5);
   values.push(6);
   println!("{values:?}");

   values.pop();
   println!("{values:?}");
}

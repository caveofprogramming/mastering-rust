
/* 
 * Each value has an owner.
 * A value can only have one owner at a time.
 * When the owner goes out of scope, the value is dropped.
 */

#[derive(Debug, Copy, Clone)]
struct Test;

fn main() {
   let s1 = Test;
   let s2 = s1;

   println!("{s1:?}");
   println!("{s2:?}");
}

fn main() {
   let mut no = 1532; // number
   let mut rev = 0; // no. after reversing
   let mut rem;

  while no >0 {
       rem = no %10;
       rev = rev*10 + rem;
       no/= 10;
        }
         print!("{}",rev );
        
}
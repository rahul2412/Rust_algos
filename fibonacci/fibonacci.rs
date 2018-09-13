fn main() {
   let n=7; // upto how many numbers
   let mut t1=0; //first no.
   let mut t2=1; // second number
   let mut next_term; 
   let mut i=0;

  loop {
        print!("{} ", t1);
        next_term = t1 + t2;
        t1 = t2;
        t2 = next_term;
        i+=1;
        if i==n
        {
        break;
        }
        
    }
}
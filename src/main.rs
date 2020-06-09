
//mod adder;

/*
fn foo(a:i32) -> Result<i32,&'static str>{

    match a {
       10  => Ok(10),
       _   => Err("value"),

    }
}*/

#[derive(Debug)]
struct Multiplication_table{
  //5*1
  first_number:i32,
  second_number:i32,
  
}


impl Multiplication_table{


  fn new(first_number:i32,second_number:i32) -> Multiplication_table{
    {
      Multiplication_table{
        first_number,
        second_number,
      }
    }
  }

}

fn multiply() -> Multiplication_table{
      Multiplication_table{
          first_number:3,
          second_number:1,
      }
}

impl Iterator for Multiplication_table{

  type Item = i32;

  fn next(&mut self) -> Option<Self::Item>{
     
     let prev = self.second_number;
//     println!("prev = {}",prev);
     self.second_number += 1;
     if prev <= 10{
            Some(self.first_number * prev)
     }else{
            None
     }

  }
}




fn main(){
  let a = 10;  
 

 let mut table1 = Multiplication_table::new(2,1) ;
/*
 for r in table1.take(20){
        println!("{:?}",r);
 }
*/
 
 for (i,j) in table1.take(5).enumerate(){
        println!("{},{}",i,j);
 
 }
}

#[cfg(test)]
    mod test_main{

        use super::*;
        #[test]
        fn  multiplication_table_2(){
              let mut table1 = Multiplication_table::new(2,1) ;
              for (i,j) in table1.take(10).enumerate(){
                    //  println!("{},{}",i,j);
                    //1*2,2
                    //2*2,4
                    //3*2,6
                  assert_eq!(j as usize,2*(i+1));
              }
        }   

        #[test]
        #[ignore]
        fn  multiplication_table_3(){
            let mut table3 = Multiplication_table::new(3,1) ;
            for (i,j) in multiply().take(10).enumerate(){
              //  println!("{},{}",i,j);
                assert_eq!(j as usize,3*(i+1));
            }    
        } 
}
/*
        
*/
//}
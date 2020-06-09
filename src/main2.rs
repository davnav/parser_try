
#[derive(Clone,PartialEq,Debug,Eq)]
struct Element{
    name: String,
    attributes: Vec<(String,String)>,
    children: Vec<Element>,
}

struct foo{
    x:i32,
    y:i32,
}

#[derive(Debug,Eq,PartialEq)]
enum Myerror<'a>{
    Numerr(&'a str),
    Strerr(&'a str),
}


fn the_letter_a(input: &str) -> Result<(&str,()),Myerror>{

     match input.chars().next(){
         Some('a') => Ok((&input[1..],())),
         Some('1') => Ok((&input[1..],())),
         _ => Err(Myerror::Numerr("not a number")),
     }
    

}


// fn match_literal(expected: &'static str) ->
//     impl Fn(&str) -> Result<(&str,()),&str>
// {
//     move |input| match input.get(0..expected.len()){
//         Some(next) if next == expected =>{
//                                             Ok((&input[expected.len()..],()))
                                        
//                                         }
//         _ => Err(input),
//     }
// }   



fn main() {
    let a = 1;
    let b = 3;
    let c = 2;
    let d = 15;
    
//     let good_result: Result<i32, &str> = Ok(10);
//     let bad_result: Result<i32, &str> = Err("error happened");

//     match good_result{
//         Ok(x) => { println!("result is {}",x) },
//         Err(e) => { println!("err is {}",e)},
//     }

//     match bad_result{
//         Ok(x) => { println!("result is {}",x) },
//         Err(e) => { println!("{}",e)},
//     }


//     let mut exp = 10;
  
//     let is_reference = &5;
//     let ref is_refer_exp = exp;
//     match is_reference {
//         &val => { println!("value from reference = {}",val) }
//     };
//    match *is_reference {
//         val => { println!("value from reference by dereferencing = {}",val) }
//     }
    

//     match is_refer_exp{
//         val => { println!("value from reference with ref keyword = {}",val) }
//     }
  
    

//     match exp{
//        10 => {  println!("it is 10")},
//        _  => { println!("some other number")},
//    }

//    match exp{
//        ref mut r => { *r = 11 ; println!("mutable reference for exp and changing the value   = {:?}",r)}
//    }

//    //println!("r = {}",r);

//    let f = foo{x:1,y:3};
//    match f{
//        foo{x,y} => { println!("{},{}",x,y)}
//    }
//    println!("{}",exp);

//     let op:Option<i32> =  Some(20) ;
//     match op{
//         None => { println!("not a value")},
//         Some(x) if b == c  => { println!("match successful {}", x) } ,
//      //   Some('a') => { println!("match second match exp successful {}", a.unwrap())},
//         Some(_) => println!("this is matching b is not eq c"),
        
//     };

    
    
//     fn add(b:i32,c:i32) -> impl Fn(i32,i32) -> Option<i32> { 
//      |b:i32,c:i32| -> Option<i32> { if b == c {Some(4)} else { None } } 
//     }

//   // println!("{:?}",add(b,c));
//     let st = "a@veen";
// //    let a = the_letter_a(st);
// //    println!("{:?}",a);

}

//#[test]
// fn literal_parser(){
//     let parse_joe = match_literal("hello Joe");

//     assert_eq!(Ok(("",())),parse_joe("hello Joe") );

//     assert_eq!(Ok(("",())),parse_joe("hello Joe hello naveen") );
// }   

#[test]
fn test_with_the_letter_a(){
  //  let parse_joe = the_letter_a("123");
  //  use super::*;
    assert_eq!(Ok(("23",())),the_letter_a("123") );
    assert_eq!(Ok(("123",())),the_letter_a("a123") );
    assert_eq!(Err(Myerror::Numerr("not a number")),the_letter_a("b123") );
    //assert_eq!(Ok(("",())),parse_joe("hello Joe hello naveen") );
}   


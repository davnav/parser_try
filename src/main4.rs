
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
/*
fn the_letter_a(input: &str) -> Result<(&str,()),&str>{

     match input.chars().next(){
         Some('a') => Ok((&input[1..],())),
         _ => Err(input),
     }
    

}
*/

fn match_literal(expected: &'static str) ->
    impl Fn(&str) -> Result<(&str,()),&str>
{
    move |input| match input.get(0..expected.len()){
        Some(next) if next == expected =>{
                                            Ok((&input[expected.len()..],()))
                                        
                                        }
        _ => Err(input),
    }
}   


fn identifier(input:&str) -> Result<(&str,String),&str> {

    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next()  {
            Some(next) if next.is_alphanumeric() => { matched.push(next) },
            _ => return Err(input),
    }

    while let Some(next) = chars.next(){

            if next.is_alphanumeric() || next == '-' {
                matched.push(next);
            }else{
                break;
            }

    } 


    let match_index = matched.len();

    Ok((&input[match_index..],matched))
}




fn main() {
    

    
/*    
    fn add(b:i32,c:i32) -> impl Fn(i32,i32) -> Option<i32> { 
     |b:i32,c:i32| -> Option<i32> { if b == c {Some(4)} else { None } } 
    }
*/

  // println!("{:?}",add(b,c));
    let st = "a@veen";
//    let a = the_letter_a(st);
//    println!("{:?}",a);
}


#[test]
fn literal_parser(){
    let parse_joe = match_literal("hello Joe");

    assert_eq!(Ok(("",())),parse_joe("hello Joe") );
 //   assert_eq!(Ok(("",())),parse_joe("hello Jo1") );

    assert_eq!(Ok((" hello naveen",())),parse_joe("hello Joe hello naveen") );
}   

fn pair<P1,P2,R1,R2>(parse1:P1,parse2:P2) -> impl Fn(&str) ->
    Result<(&str,(R1,R2)),&str>

    where P1:Fn(&str) -> Result<(&str,R1),&str>
          P2:Fn(&str) -> Result<(&str,R2),&str>
    {

    }


#[test]
fn identifier_parser(){
    
    assert_eq!(Ok(("","I-am-an-identifier".to_string())),identifier("I-am-an-identifier"));

    assert_eq!(Ok(("I-am-an","!_identifier".to_string())),identifier("I-am-an!_identifier"));
    assert_eq!(Ok(("entirely an identifier","not".to_string())),identifier("not entirely an identifier"));
}
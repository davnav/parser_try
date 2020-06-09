fn parse<T>(mut parser:T,input:&str) -> Result<T::output,ParseError> 
where 
    T:Parse 
{
    
    match parser.parse(input){
        Ok((parsed,remaining)) => {
            if remaining.is_empty(){
                Ok(parsed)
            }else{
                Err(ParseError::UnexpectedEndofInput)
            }
        }
        Err(parse_error) => { Err(parse_error)}
    }
}

#[derive(Debug,PartialEq,Eq)]
enum ParseError{
    UnexpectedEndofInput,
    Msg(String),
}

trait Parse:Sized{
    type output;
    
    fn parse<'a>(&mut self,input:&'a str) -> Result<(Self::output,&'a str),ParseError>;
    
    fn map<F,B>(self,map_fun:F) ->  Map<Self,F> 
    where F: Fn(Self::output) -> B
    {
        Map{
            parser:self,
            f:map_fun,
        }

    }
}

#[derive(Debug)]
struct NumberParser;

fn number() -> NumberParser{
    NumberParser
}



impl Parse for NumberParser{
    
    type output = i32;
    fn parse<'a>(&mut self,input:&'a str) -> Result<(Self::output,&'a str),ParseError>{
        let numbers =  input.chars().take_while(|c| c.is_numeric()).collect::<String>();
        let leng = numbers.len();
        let number = numbers.parse::<i32>().unwrap();
        Ok((number,&input[leng..]))

       }
      //  todo!("parse for i32")
    
}


    
#[derive(Debug)]
struct Map<P,F>{
    parser:P,
    f:F,
}


impl<P,F,B> Parse for Map<P,F> 
where 
    P:Parse,
    F:Fn(P::output) -> B,
{
    
    type output = B;
    fn parse<'a>(&mut self,input:&'a str) -> Result<(Self::output,&'a str),ParseError>{
        
        match self.parser.parse(input){
            Ok((parsed,remaining)) => {
                Ok(((self.f)(parsed),remaining))
            },
            Err(_) => {Err(ParseError::UnexpectedEndofInput)},
        }
     //   todo!()
        
    }
      //  todo!("parse for i32")
    
}



fn main(){

}

#[test]
//use super::*
fn parsr_number(){
    let (number,remaining) = number().parse("123").unwrap();
    assert_eq!((123,""),(number,remaining) );
}
#[test]
fn parsr_number_with_reminder(){
    let (number,remaining) = number().parse("123a").unwrap();
    assert_eq!(123,number );
}

#[test]
fn parsr_unexpected(){
    assert_eq!(Err(ParseError::UnexpectedEndofInput),parse(NumberParser,"112a") );
}

#[test]
fn map_test(){

    let (number,remaining) = NumberParser.map(|n| n+1).parse("123").unwrap();
    assert_eq!((124,""),(number,remaining) );
 
}


#[derive(Debug)]
struct Command<'a>{

    cmd:&'a str,
    path:Option<&'a str>,
       
}

impl<'a> Command<'a> { 
    fn new(cmd:&'a str) -> Self{
            Command{
                cmd,
                path:None,
            }
    }

    fn execute(&mut self) -> &mut Self{
        println!("executing the command:{:?}",&self);
        self
    }

    fn execute_chain(&mut self) -> &mut Self{
        println!("executing chain the command:{:?}",&self);
        self.path = Some("/root");
        println!("cmd:{:?}",&self);
        self
    }

    fn string_chain(&self,str_passed:&str) -> &Self{
        println!("string literal:{:?}",str_passed);
        self
    }
}



fn main(){
  let mut cmd = Command::new("ls");
  cmd.execute().execute_chain().string_chain("naveen");
   cmd.execute().execute_chain().string_chain("newone");
  //execute_chain().string_chain("naveen");

}
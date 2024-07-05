use std::{
    fmt::Debug, fs, io::{Seek, Write}, net::ToSocketAddrs, ops::Add
};

fn main() {
    let mut xml = Xml::new("index.xml");
    
}
struct Xml {
    file: XmlFile,
}

impl Xml {
    fn new(path: &str) -> Self {
        Self {
            file: XmlFile::new(path),
        }
    }
    fn write(&mut self, tag: &str, headers: Option<Vec<(&str, &str)>>, inside: Option<&str>) {
        let mut retString = match headers {
            Some(s) => [
                tag,
                s.into_iter()
                    .map(|x| -> String { [" ", x.0, " = \"", x.1, "\""].concat() })
                    .collect::<String>()
                    .as_str(),
            ]
            .concat(),
            None => tag.to_string(),
        };
        let tk = match inside {
            Some(s) => TokenEnum::Inline(retString, s.to_string()),
            None => TokenEnum::Multiline(retString),
        };
        println!("{:#?}", tk);
        self.file.write(tk);
    }
}

enum TokenEnum {
    Inline(String, String),
    Multiline(String),
}

struct XmlFile {
    file: fs::File,
    stack: Vec<String>,
    n: usize
}

impl Drop for XmlFile {
    fn drop(&mut self) {
        self.close_all();
    }
}
impl XmlFile {
    // fn wrt_mono(header: String) => {write_pretty(TokenEnum::Mono(header))};
    fn new(path: &str) -> Self {
        Self {
            file: fs::File::create(path).unwrap(),
            stack: Vec::new(),
            n: 1
        }
    }
    fn write(&mut self, token: TokenEnum) {
        let a = match token {
            TokenEnum::Inline(header, inside) => format!(
                "<{}>{}</{}>\r\n",
                header,
                inside,
                header.split(' ').next().unwrap()
            ),
            TokenEnum::Multiline(header) => {
                // self.file.write(["<",s.as_str(),">\r\n"].concat().as_bytes()).unwrap();
                self.stack
                    .push(String::from(header.split(' ').next().unwrap()));
                format!("<{header}>\r\n")
            }
        };
        self.file.write(a.as_bytes()).unwrap();
    }
    fn close_last(&mut self) {
        let pop_return = self.stack.pop();

        match pop_return {
            Some(header) => {
                self.file
                    .write(format!("{}\r\n", ["</", header.as_str(), ">"].concat()).as_bytes())
                    .unwrap();
            }

            // Some(s) => self.file.write(["</",s.as_str(),">\r\n"].concat().as_bytes()).unwrap(),
            None => panic!("stack is empty"),
        };
    }
    fn close_all(&mut self){
        while self.stack.len() > 0
        {
            self.close_last();
        }
    }
  
}

// fn close(&mut self){
//     let mut pop_return = self.stack.pop();

//     while  let Some(a) = pop_return{

//     }
// }
  // fn close_all_pretty(&mut self){
    //     while self.stack.len() > 0
    //     {
    //         self.close_last_pretty();
    //     }
    // }
    // fn write_pretty(&mut self, token: TokenEnum) {
    //     self.file.write("\t".repeat(self.stack.len()*self.n).as_bytes()).unwrap();
    //     self.write(token);
    // }
    // fn close_last_pretty(&mut self) {

    //     self.file.write("\t".repeat((self.stack.len()-1)*self.n).as_bytes()).unwrap();
    //     self.close_last();   
    // }

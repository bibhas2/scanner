pub struct Scanner<'a> {
    chars:std::str::Chars<'a>,
    word:String
}
 
impl <'a> Scanner<'a> {
    pub fn new(line : &'a str) -> Scanner {
        Scanner {
            word: String::new(),
            chars: line.chars()
        }
    }
    
    pub fn next<T>(&mut self) -> Option<T> 
        where T : std::str::FromStr
    {
        self.word.truncate(0);
        
        let mut start_mode = true;
        
        //Eat space
        loop {
            match self.chars.next() {
                Some(ch) => {
                    if ch.is_whitespace() {
                        if start_mode == true {
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        start_mode = false;
                        self.word.push(ch);
                    }
                },
                None => {
                    break;
                }
            }
        }
        
        if self.word.len() == 0 {
            return None;
        }
        
        match self.word.parse::<T>() {
            Ok(val) => return Some(val),
            _ => return None
        }
    }
}
#[test]
fn test1() {
    let line = "  12 34.56  ";
    let mut s = Scanner::new(line);
    
    let i_val:Option<i64> = s.next();
    assert!(i_val == Some(12));
    
    let f_val:Option<f64> = s.next();
    assert!(f_val == Some(34.56));

    let no_val:Option<f64> = s.next();
    assert!(no_val == None);
}
#[test]
fn test2() {
    let line = "  12 34.56  ";
    let mut s = Scanner::new(line);
    let f_val:Option<f64> = s.next();
    //Integer is parsed as float just fine
    assert!(f_val == Some(12.0));
}

#[test]
fn test3() {
    let line = "  34.56  ";
    let mut s = Scanner::new(line);
    let i_val:Option<i64> = s.next();
    //Float can not be parsed as Integer
    assert!(i_val == None);
}

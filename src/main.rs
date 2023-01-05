use substring::Substring;
use ascii_converter::*;
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut data: Vec<i8> = [0; 30000].to_vec();
    let mut save_me: Vec<u8> = vec![0];
    let mut argindex: usize = 0;
    let mut index: usize = 0;
    let mut skip: usize = 0;
    let mut help_me = "".to_string();
    let file = &args[1];
    let mut contents = std::fs::read_to_string(file)
        .expect("Could not read file");
    remove_whitespace(&mut contents);
    for (x,y) in contents.chars().enumerate() {
        if x > skip.try_into().unwrap() {
        if y == '[' {
            while(data[index] > 0){
            for (x,y) in contents.substring(x,contents.find(&"]".to_string()).expect("Oh")).chars().enumerate() {
                if x > skip.try_into().unwrap() {
                if y == '[' {
                    
                while(data[index] > 0){
                for (x,y) in contents.substring(x,contents.find(&"]".to_string()).expect("Oh")).chars().enumerate() {
                if x > skip.try_into().unwrap() {
                        if y == '>' {index+=1;}
                        if y == '<' {index-=1;}
                        if y == '+' {data[index]+=1;}
                        if y == '-' {data[index]-=1;}
                        if y == ',' {help_me = args[argindex].clone();data[index] = string_to_decimals(&help_me).unwrap()[0] as i8;argindex+=1;}
                        if y == '.' {if data[index] != 0{save_me[0] = data[index] as u8;print!("{}",format!("{:?}",decimals_to_string(&save_me)).substring(4,5));}}
                        if index > 30000 {index = 0;}
                        if index < 0 {index = 30000;}
                        }
                    }
                    
                }
                if y == '>' {index+=1;}
                if y == '<' {index-=1;}
                if y == '+' {data[index]+=1;}
                if y == '-' {data[index]-=1;}
                if y == ',' {help_me = args[argindex].clone();data[index] = string_to_decimals(&help_me).unwrap()[0] as i8;argindex+=1;}
                if y == '.' {if data[index] != 0{save_me[0] = data[index] as u8;print!("{}",format!("{:?}",decimals_to_string(&save_me)).substring(4,5));}}
                if index > 30000 {index = 0;}
                if index < 0 {index = 30000;}
                }
            }
        }
        }
        if y == '>' {index+=1;}
        if y == '<' {index-=1;}
        if y == '+' {data[index]+=1;}
        if y == '-' {data[index]-=1;}
        if y == ',' {help_me = args[argindex].clone();data[index] = string_to_decimals(&help_me).unwrap()[0] as i8;argindex+=1;}
        if y == '.' {if data[index] != 0{save_me[0] = data[index] as u8;print!("{}",format!("{:?}",decimals_to_string(&save_me)).substring(4,5));}}
        if index > 30000 {index = 0;}
        if index < 0 {index = 30000;}
        }
    }
}

use std::fs;
fn main() {
    let path = "C:/Users/user/projects/rust_new_year_task1/rust_task1/src/file.txt";
    let x = fs::read_to_string(path).unwrap();
    
    let mut sum:u128 = 0;
    
    for line in x.lines(){
        let mut temp= String::new();
        &line.replace("one", "1").replace("two", "2").replace("three", "3").replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7").replace("eight", "8").replace("nine", "9");
        
        for i in line.chars(){
            if i == '1'|| i == '2'|| i == '3'|| i == '4'|| i == '5'
            || i == '6' || i == '7' || i == '8' || i == '9' {
                temp += &i.to_string();
            }
        }
        let mut value = String::new();
        value.push(temp.as_bytes()[0] as char);
        value.push(temp.as_bytes()[temp.len()-1] as char);
        sum = sum + temp.parse::<u128>().unwrap();
    }
    println!("{}", sum);
}

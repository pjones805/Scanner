use std::io;



// Input Output
pub fn num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn flo() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn tex() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn cha() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().next().unwrap()
}



// multi Input Output
pub mod multi {
    use std::io;
    use std::convert::TryInto;


    pub fn nums(l: u16) -> Vec<i32> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();


        // Make sure that we don't get errors because we try to pull a 
        // number that doesn't exsit
        while nums.len() < l.try_into().unwrap() {
            nums.push(0);
        }
        nums
    }

    
    pub fn flos(l: u16) -> Vec<f64> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut flos: Vec<f64> = input  
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect(); 

        
        while flos.len() < l.try_into().unwrap() {
            flos.push(0.0);
        }
        flos
    }


    pub fn texs(l: u16) -> Vec<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut texs: Vec<String> = input  
            .split_whitespace()
            .map(String::from)
            .collect();

        
        while texs.len() < l.try_into().unwrap() {
            texs.push("null".to_string());
        }
        texs
    }
}

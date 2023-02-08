//mod print; file name import way
fn main(){
    //if files are divided,
    //print::run();
    run();
}
pub fn run(){
    //Print to console
    println!("Hello From the print rs function");
    
    //Basic Formatting
    println!("Number: {}",1);
    println!("{} is from {}","Khine","Japan");
    
    //Positional Arguments
    println!("{0} is from {1} and {2} likes to {} ","Khine","Japan","Chocolate");
    
    //Named Arguments
    println!("{name} likes to play {activity}",name="John",activity="Baseball!");
    
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal:{:o}",10,10,10);
    
    //Placeholder for debug trait
    println!("{:?}",(12,true,"hello"));//tuple
    
    //Basic math
    println!("10+10={}",10+10);
}

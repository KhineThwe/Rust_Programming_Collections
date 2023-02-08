//mod print; file name import way
fn main(){
    //if files are divided,
    //print::run();
    run();
}
pub fn run(){
    //variables
    //rust is also a block-scoped language
    let name = "Khine";
    let mut age = 24;//mut means mutable,can change
    age = 25;
    println!("my name is {} and I am {}",name,age);
    
    //Define constant
    const ID: i32 = 001;
    println!("ID: {} ",ID);
    
    //assign multiple vars
    let (my_name,my_age) = ("Khine",24);
    println!("my name is {} and I am {}",my_name,my_age);

}

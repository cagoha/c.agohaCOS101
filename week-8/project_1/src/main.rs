use std::io;

fn office_admin(name: &str){
    let admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let public_servant = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    
        println!("Select position: ");
        for x in 0..public_servant.len(){
            println!("{}. {}",x+1,admin[x]);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let position:usize = input.trim().parse().expect("Invalid input");
        
        if position >= 1 && position <= public_servant.len() {
             println!("{}, You are an Office Admin {} {}",name,admin[position - 1],public_servant[position - 1]);
        }
         else{
                println!("You must choose from the list provided!");
             }

}
fn academic(name: &str){
    let academics = vec!["(-)","Research assistant","PhD Canditate","Post-Doc Researcher","Senior Researcher","Dean"];
    let public_servant = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
 
  println!("Select position: ");
        for x in 0..public_servant.len(){
            println!("{}. {}",x+1,academics[x]);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let position:usize = input.trim().parse().expect("Invalid input");
        
        if position >= 1 && position <= public_servant.len(){
             println!("{}, You are an Academic {} {}",name,academics[position - 1], public_servant[position - 1]);
        }
         else{
                println!("You must choose from the list provided!");
             }
}
fn lawyer(name: &str){
     let law = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
     let public_servant = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    
     println!("Select position: ");
        for x in 0..public_servant.len(){
            println!("{}. {}",x+1,law[x]);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let position:usize = input.trim().parse().expect("Invalid input");
        
        if position >= 1 && position <= public_servant.len(){
             println!("{}, You are a {} {} Lawyer",name, law[position - 1],public_servant[position - 1]);
        }
         else{
                println!("You must choose from the list provided!");
             }
}
fn teacher(name: &str){
     let teaching = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
     let public_servant = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    
     println!("Select position: ");
        for x in 0..public_servant.len(){
            println!("{}. {}",x+1,teaching[x]);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let position:usize = input.trim().parse().expect("Invalid input");
        
        if position >= 1 && position <= public_servant.len(){
             println!("{}, You are a {} {} in the teaching Department",name, teaching[position - 1],public_servant[position - 1]);
        }
         else{
                println!("You must choose from the list provided!");
             }
}
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("Failed to Read Input");
    let name:String = input1.trim().parse().expect("Invalid input");

    println!("Choose any of the Categories:");
    println!("\n1. Office Admin department");
    println!("\n2. Academic department");
    println!("\n3. Law department");
    println!("\n4. Teaching department");

    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let number:u8 = input2.trim().parse().expect("Invalid input");
    if number == 1{
      office_admin(&name);
    }
    else if number == 2{
    academic(&name);
    }
    else if number == 3{
    lawyer(&name);
    }
    else if number == 4{
    teacher(&name);
    }
    else{
        println!("You are to pick from the list!");
    }
}
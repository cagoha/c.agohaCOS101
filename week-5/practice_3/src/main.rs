fn main() {
    let name1:&str = "Ayomide Adesokan";
    println!("My name is {}",name1 );

    let name2 = name1.replace("Ayomide","Adebare");
    println!("But you can also call me {}",name2 );

    let faculty = "Faculty of Science and Technology";
    let school = faculty.replace("Faculty","School");
    println!("I am a student of the {}",school );
}
struct Employee{
    ceo:String,
    company:String,
    age:u32,
}
fn main() {
    let emp1 = Employee{
        ceo: String::from("Satya Nadela"),
        company:String::from("Microsoft Cooperation"),
        age:56
   };
   let emp2 = Employee{
    ceo:String::from("Sundai Pichai"),
    company:String::from("Google Inc"),
    age:51
   };

   display(emp1);
   display(emp2);


}
fn display(emp:Employee){
    println!("Name is: {}, Company is: {}, Age is: {}",emp.ceo, emp.company, emp.age );
}
use std::io::Write;

fn main() {

let column1 = vec![
                   "STUDENT NAME,",
                   "Oluchi Mordi,",
                   "Adams Aliyu,",
                   "Shanai Bolade,",
                   "Adekunle Gold,",
                   "Blanca Edemoh,"
                   ];
let column2 = vec![
                   "MATRIC NUMBER,",
                   "ACC10211111,",
                   "ECO10110101,",
                   "CSC10328828,",
                   "EEE11020202,",
                   "MEE10202001,"
                   ];
let column3 = vec![
                   "DEPARTMENT,",
                   "Accounting,",
                   "Economics,",
                   "Computer,",
                   "Electrical,",
                   "Mechanical,"
                   ];
let column4 = vec![
                   "LEVEL\n",
                   "300\n",
                   "100\n",
                   "200\n",
                   "200\n",
                   "100\n"
                   ];

let mut smis = std::fs::File::create("PAU SMIS.csv").expect("Failed to create code");
smis.write_all("Welcome to PAU Student Management Information System!\n".as_bytes()).expect("Failed to write");

for i in 0..column4.len(){
smis.write_all(column1[i].as_bytes()).expect("Failed to write");
smis.write_all(column2[i].as_bytes()).expect("Failed to write");
smis.write_all(column3[i].as_bytes()).expect("Failed to write");
smis.write_all(column4[i].as_bytes()).expect("Failed to write");
}
println!("File successfully created");
}



use std::io::Write;

fn main() {
    

/*let heading = vec!["STUDENT NAME,","MATRIC NUMBER,","DEPARTMENT,","LEVEL,"];
let row1 = vec!["Oluchi Mordi,","ACC10211111,","Accounting,","300,"];
let row2 = vec!["Adams Aliyu,","ECO10110101,","Economics,","100,"];
let row3 = vec!["Shanai Bolade,","CSC10328828,","Computer,","200,"];
let row4 = vec!["Adekunle Gold,","EEE11020202,","Electrical,","200,"];
let row5 = vec!["Blanca Edemoh,","MEE10202001,","Mechanical,","100,"];*/
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
}
/*smis.write_all(heading[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(heading[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(heading[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(heading[3].as_bytes()).expect("Failed to write");
smis.write_all(",\n".as_bytes()).expect("Failed to write");

smis.write_all(row1[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row1[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row1[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row1[3].as_bytes()).expect("Failed to write");
smis.write_all(",\n".as_bytes()).expect("Failed to write");

smis.write_all(row2[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row2[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row2[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row2[3].as_bytes()).expect("Failed to write");
smis.write_all(",\n".as_bytes()).expect("Failed to write");

smis.write_all(row3[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row3[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row3[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row3[3].as_bytes()).expect("Failed to write");
smis.write_all(",\n".as_bytes()).expect("Failed to write");

smis.write_all(row4[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row4[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row4[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row4[3].as_bytes()).expect("Failed to write");
smis.write_all(",\n".as_bytes()).expect("Failed to write");

smis.write_all(row5[0].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row5[1].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row5[2].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");

smis.write_all(row5[3].as_bytes()).expect("Failed to write");
smis.write_all(",".as_bytes()).expect("Failed to write");


println!("File successfully created");
}
*/

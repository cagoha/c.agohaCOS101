use std::io::Write;

fn main() {
    let commisioner = vec![
                           "Commisioner,",
                           "Aigbogun Alamba Daudu,",
                           "Murtala Afeez Bendu,",
                           "Okorocha Calistus Ogbona,",
                           "Adewale Jimoh Akanbi,",
                           "Osazuwa Faith Etieye,"
                           ];
    let ministry = vec![
                        "Ministry,",
                        "Internal Affairs,",
                        "Justice,",
                        "Defense,",
                        "Power & Steel,",
                        "Petroleum,"
                        ];
    let zone = vec![
                    "Zone\n",
                    "South West\n",
                    "North East\n",
                    "South South\n",
                    "South West\n",
                    "South East\n"
                    ];

    let mut merge = std::fs::File::create("EFCC-merged backup file.csv").unwrap();
     for i in 0..zone.len(){
     merge.write_all(commisioner[i].as_bytes()).unwrap();
     merge.write_all(ministry[i].as_bytes()).unwrap();
     merge.write_all(zone[i].as_bytes()).unwrap();
    }
    println!("Data sets successfully merged to file");
}   
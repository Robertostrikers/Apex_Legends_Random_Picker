pub mod randomizer {
    use rand::Rng;
    use std::path::Path;
    use std::fs::File;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Legend{
        name: String,
        portrait: String

    }

    //static legends_db: Vec<Legend> = legends_vector();

    pub fn random_pick() -> (String, String) {
        //temporal global in local scop
        let legends_db: Vec<Legend> = legends_vector();

        //Make struct with img and str
        let random_num = rand::thread_rng().gen_range(0..legends_db.len());
        (legends_db[random_num].name.clone(), legends_db[random_num].portrait.clone())

        //TODO avoi tostring and store String instead of &st

    }
    //read from json file the legends and store them in an vector
    pub fn legends_vector() -> Vec<Legend> {

        let json_file_path = Path::new("src/Legends/Legends.json");
        let file = File::open(json_file_path).expect("file not found");

        let json_legends:Vec<Legend> = serde_json::from_reader(file).expect("Error while reading");
        
        json_legends
    }


}




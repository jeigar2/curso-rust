use csv::{ReaderBuilder, StringRecord};
use std::{fs, collections::HashMap};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";
const SITUATION: &str = "SITUACION";
const OPTION: &str = "OPCION";

#[derive(Debug)]
struct HistoryData{
    data_type: String,
    tag: String,
    _text: String,
    life: i32,
    options: Vec<HistoryData>
}

impl HistoryData {
    fn new(row: StringRecord) -> HistoryData{

        let life = row.get(3).unwrap().trim();
        let life : i32 = life.parse().unwrap_or(0);
        return HistoryData {
            data_type:row.get(0).unwrap().trim().to_string(),
            tag:row.get(1).unwrap().trim().to_string(),
            _text:row.get(2).unwrap().trim().to_string(),
            life:life,
            options: vec![]
        };
    }
}

fn main() {
    let mut life = 100;
    let mut current_tag = FIRST_TAG;
    let mut last_record: String = "".to_string();
    // let mut history_datas: Vec<HistoryData> = vec![];
    let mut history_datas: HashMap<String, HistoryData> = HashMap::new();

    let content: String = fs::read_to_string(FILENAME).unwrap();
    // println!("{}", content);

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryData::new(result);
        let record_tag = data.tag.clone();
        if data.data_type == SITUATION {
            history_datas.insert(record_tag.clone(), data); // TODO el clone está repetido.
            last_record = record_tag;
        } else if data.data_type == OPTION {
            if let Some(_data) = history_datas.get_mut(&last_record) {
                (*_data).options.push(data);
                // println!("{}",(*_data).tag);
            }

        }

        // let life = result.get(3).unwrap().trim();
        // let life : i32 = life.parse().unwrap_or(0);

        // let _data = HistoryData {
        //     data_type:result.get(0).unwrap().trim().to_string(),
        //     tag:result.get(1).unwrap().trim().to_string(),
        //     _text:result.get(2).unwrap().trim().to_string(),
        //     life:life
        // };
        //history_datas.push(data);
        
    }

    // Game loop
    loop {
        println!("Tienes {} de vida", life);
        if let Some(data) = history_datas.get(current_tag) {
            println!("{}", data._text);
            for (index, option) in data.options.iter().enumerate() {
                println!("[{}] {}", index, option._text);
            }
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selection_option) = &data.options.get(selection){
                current_tag = &selection_option.tag;
            // } else if (selection == 77) {
            //     break;
            } 
            else {
                println!("Comando no valido");
            }
            if data.life < 0 {
                println!("¡Oh!, Tu salud merma");
            }
            life += data.life;
            println!("");
        } else {
            break;
        }

        if life <= 0 {
            println!("Se acabo el juego, no te queda vida");
            break;
        }
    }

    //println!("{:?}", history_datas);
    // println!("\n-----\n{:?}\n-----\n", history_datas["DERECHA"]);

}

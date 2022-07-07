

static CAP: [i64; 5] = [112414000, 123655000, 133000000, 139000000, 150000000];
static TAX: [i64; 5] =  [136606000, 150267000, 161623153, 168914423, 182281752];
static VETMAX: [i64; 5] = [39344900, 43279250, 46550000, 48650000, 52500000];
static MIDMAX: [i64; 5] = [39344900, 43279250, 46550000, 48650000, 52500000];
static ROOKMAX: [i64; 5] = [39344900, 43279250, 46550000, 48650000, 52500000];
static MLE: [i64; 5] = [9536000, 10489566, 11282296, 11791272, 12724394];
static TAXMLE: [i64; 5] = [5890000, 6478979, 6968616, 7282990, 7859341];

use csv::{Reader, StringRecord, Writer, Error};
use std::process;
use std::collections::HashMap;

/// Internal data set to make aggregation simpler
#[derive(Debug)]
struct DataSet {
    /// Header row of CSV file
    headers: StringRecord,

    /// Records from CSV file
    records: Vec<StringRecord>,
}

/// Reads csv data from a file and returns a DataSet
fn read_from_file(path: &str) -> Result<DataSet, Box<Error>> {
    let mut reader = Reader::from_path(path)?;
  
    let headers = reader.headers()?.clone();
  
    let records = reader
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;
  
    Ok(DataSet { headers, records })
  }

// team has books for next 5 years
// each player is vector 5 entries
struct Team {
    team: String,
    salary: i64,
    books: Vec<Player>,
    used_mle: bool,
    years_in_tax: i8,
}

// extern crate serde_derive;

// use std::error::Error;

#[derive(Deserialize, Clone, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct Player {
    Player: String,
    Age: i8,
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    Signed: String,
    Guaranteed: String,
    Team: String,
    Position: String,
    EXP: String,
    TeamAbbr: String,
    Amount: String,
    option_type: String,
    option_year: String
    // checked:bool
    // contract: Vec<i64>,// next 5 years
    // exp: i8, //years experience (age - 20 for now)
}

struct Trade {
    team1: Team,
    team2: Team,
    players1: Vec<Player>,
    players2: Vec<Player>
}

// load these players into teams



// exception list : 
//Qualifying Veteran Free Agent Exception
//Early Qualifying Veteran Free Agent Exception
//Non-Qualifying Veteran Free Agent Exception
//Traded Player Exception (dumb)
//MLE
//TAX MLE
//ROOM
//BI-ANNUAL
//ROOKIE
//MINIMUM
//DISABLED PLAYER

fn use_mle(team: String, player: Player) {
    // under tax above cap
    // under mle[1]
    // not used already
}

fn use_tax_mle() {
    // above tax
    // under mle[1]
    // not used already
}

fn use_minimum() {
    // contract is a minimum
}

fn use_room(){
    // not already used (tax or regular)
    //below cap
}

fn use_qual_vet_fa(){

}

fn use_early_qual_vet_fa(){

}

fn use_non_qual_vet_fa(){

}

#[tauri::command]
pub fn get_team_cap_sheet(team: String, data: String) -> Vec<Player> {
    let master_cap_sheet: HashMap<String, Player> = get_cap_dict(data);




    let m: Vec<Player> = master_cap_sheet.values().cloned().collect::<Vec<Player>>();
    let f: Vec<Player> = m.into_iter().filter(|player| player.Team == team).collect::<Vec<Player>>();
    
          



    // for (key, value) in &master_cap_sheet {
    //     println!("{}: {}", key, value.len());
    // }



    // let team = &master_cap_sheet[&team];

    // println!("{}", team[0].Player);
    return f.clone()
}

use rusty_money::{Money, Round, iso};


// this whole thing should really be in team objects
// make an hashmap of team objects -> write this to json and read from json
// all relevant info readily available
// this doesnt subtract leaving salary
use rust_decimal_macros::dec;
#[tauri::command]
pub fn trade(team1: String, team2: String, players1: Vec<Player>, players2: Vec<Player>, data: String) -> (bool, String) {

    let cap = Money::from_minor(CAP[1]*100, iso::USD);
    let master_cap_sheet: HashMap<String, Player> = get_cap_dict(data.clone());



    //add the filtering
    // let team_1_books = &master_cap_sheet[&team1];
    let team_1_books: Vec<Player> = get_team_cap_sheet(team1.clone(), data.clone());
    // let team_2_books = &master_cap_sheet[&team2];
    let team_2_books: Vec<Player> = get_team_cap_sheet(team2.clone(), data.clone());

    // println!("books {} {}", team_1_books.len(), team_2_books.len());

    // let ind = team_1_books.index_of()

    //find the players in master_cap_sheet
    //swap them

    // this would be faster with up to date team objects

    

    let mut players_salary_1 = Money::from_minor(0, iso::USD);
    for player in players1{
        if &player.one.len() < &1{
            continue
        }
        players_salary_1 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }


    // THIS NEEDS A LOT OF LOGIC TO UNDERSTAND CORRECT NUMBER
    // NEED TO WORK OUT WHAT NON GUARANTEES THERE ARE
    // find out rules for what counts towards cap
    let mut salary_team_1 = Money::from_minor(0, iso::USD);
    for player in team_1_books{
        if &player.one.len() < &1{
            continue
        }
        salary_team_1 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }

    // println!("{} {} {} {}", team1, salary_team_1, players_salary_1, cap);

    let mut players_salary_2 = Money::from_minor(0, iso::USD);
    for player in players2{
        if &player.one.len() < &1{
            continue
        }
        players_salary_2 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }

    let mut salary_team_2 = Money::from_minor(0, iso::USD);
    for player in team_2_books{
        if &player.one.len() < &1{
            continue
        }
        salary_team_2 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }

    println!("{} {} {} {} {}", team2, salary_team_2, players_salary_2, players_salary_1, cap);

    println!("{} {}", *players_salary_1.amount() * dec!(1.25) > *players_salary_2.amount(), *players_salary_2.amount() * dec!(1.25) > *players_salary_1.amount());


    if *players_salary_1.amount() * dec!(1.25) >= *players_salary_2.amount() && *players_salary_2.amount() * dec!(1.25) >= *players_salary_1.amount(){
        return (true, "Salary within 125%".to_string())
    } else {
        // here should try all of the other ways to get trades to work
        if salary_team_1+ players_salary_2 > cap &&  *salary_team_2.amount() + *players_salary_1.amount() > *cap.amount(){
            // println!("{}  <  {}", *salary_team_1.amount() + *players_salary_2.amount(), *cap.amount());
            return (false, "Insufficient matching salary. Both teams over cap".to_string())
        } else if salary_team_2 + players_salary_1 > cap {
            return (false, "Insufficient matching salary. Team 2 over cap".to_string())
        }
    }
   


    // let chi: Team = Team {
    //     team: "CHI",
    //     salary: 100000000
    // }

    // let mil: Team = Team {
    //     team: "MIL",
    //     salary: 100000000
    // }

    //check if either team goes over cap
        // if true check within 125%
            //reject if not
    
    // println!("{} {}", salary_team_1.amount + players_salary_2, salary_team_2 + players_salary_1);
    (true, "room".to_string())

}


fn main() {
    // tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![masterap_sheet])
    // .run(tauri::generate_context!())
    // .expect("error while running tauri application");



    // let brad: Player = Player {
    //     Player: "Bradley Beal".to_string(),
    //     Age: 29,
    //     zero: "$33,724,200".to_string(),
    //     one: "$37,262,300".to_string(),
    //     two: "".to_string(),
    //     three: "".to_string(),
    //     four: "".to_string(),
    //     five: "".to_string(),
    //     Signed: "".to_string(),
    //     Guaranteed: "".to_string(),
    //     Team: "Wizards".to_string(),
    //     Position: "".to_string(),
    //     EXP: "".to_string(),
    //     TeamAbbr: "".to_string(),
    //     Amount: "".to_string(),
    //     option_type: "".to_string(),
    //     option_year: "".to_string()
        

    // };
    // let russ: Player = Player {
    //     Player: "Russel Westbrook".to_string(),
    //     Age: 29,
    //     zero: "$33,724,200".to_string(),
    //     one: "$37,262,300".to_string(),
    //     two: "".to_string(),
    //     three: "".to_string(),
    //     four: "".to_string(),
    //     five: "".to_string(),
    //     Signed: "".to_string(),
    //     Guaranteed: "".to_string(),
    //     Team: "Lakers".to_string(),
    //     Position: "".to_string(),
    //     EXP: "".to_string(),
    //     TeamAbbr: "".to_string(),
    //     Amount: "".to_string(),
    //     option_type: "".to_string(),
    //     option_year: "".to_string()
    // };
    // let brad2: Player = Player {
    //     Player: "Bradley Beal".to_string(),
    //     Age: 29,
    //     zero: "$33,724,200".to_string(),
    //     one: "$37,262,300".to_string(),
    //     two: "".to_string(),
    //     three: "".to_string(),
    //     four: "".to_string(),
    //     five: "".to_string(),
    //     Signed: "".to_string(),
    //     Guaranteed: "".to_string(),
    //     Team: "Wizards".to_string(),
    //     Position: "".to_string(),
    //     EXP: "".to_string(),
    //     TeamAbbr: "".to_string(),
    //     Amount: "".to_string(),
    //     option_type: "".to_string(),
    //     option_year: "".to_string()
    // };
    // let russ2: Player = Player {
    //     Player: "Russel Westbrook".to_string(),
    //     Age: 29,
    //     zero: "$33,724,200".to_string(),
    //     one: "$37,262,300".to_string(),
    //     two: "".to_string(),
    //     three: "".to_string(),
    //     four: "".to_string(),
    //     five: "".to_string(),
    //     Signed: "".to_string(),
    //     Guaranteed: "".to_string(),
    //     Team: "Lakers".to_string(),
    //     Position: "".to_string(),
    //     EXP: "".to_string(),
    //     TeamAbbr: "".to_string(),
    //     Amount: "".to_string(),
    //     option_type: "".to_string(),
    //     option_year: "".to_string()
    // };
    // trade("Wizards".to_string(), "Heat".to_string(), [brad].to_vec(), [russ].to_vec());
    // trade("Bucks".to_string(), "Suns".to_string(), [brad2].to_vec(), [russ2].to_vec())
}

use serde::{Serialize, Deserialize};

pub fn get_cap_dict(contract_data: String) -> HashMap<String, Player>{

    let caps: HashMap::<String, Player> = serde_json::from_str(&contract_data).unwrap();

    // let caps: any = serde_json::from_str(&contract_data).unwrap();

    // println!("{}", caps);


    // let mut contract_data = match read_from_file("master_cap.csv") {
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         process::exit(1);
    //     }
    // };
    // let mut contract_data: DataSet = read_from_file("contracts.csv");

    // hashmap with cap sheet for each team
    let mut master_cap_sheet: HashMap<String, Vec<Player>> = HashMap::new();
    let mut m: HashMap<String, [Player; 20]> = HashMap::new();
    let mut cur_team: String = "".to_string();
    let mut cur_players: Vec<Player> = Vec::new();
    // for (key, player) in &caps{
    //     // // println!("{}", row.as_slice())
    //     // let r: Player = row.deserialize(None).expect("deserialize error");
   
    //     println!("{}", player.Team);
    //     // let t = serde_json::from_str(row).unwrap();

    //     m[&player.Team].push(player)
        
    //     // if cur_team != player.Team {
    //     //     println!("new team");
    //     //     if cur_team == "" {
    //     //         cur_team = player.Team.clone();
    //     //         continue
    //     //     } else {
    //     //         master_cap_sheet.insert(cur_team, cur_players.clone());
    //     //         cur_players.clear();
    //     //         cur_team = player.Team.clone();
    //     //         continue
    //     //     }
    //     // }

    //     // cur_players.push(r.clone());
        
    //     // use player.team as key for adding to hash
    //     // println!("{} {} {}", r.Player, r.one, r.Team);
    // }

    // for (i, player_vec) in master_cap_sheet {
    //     println!("{} {} {}", i, player_vec[0].Player, player_vec[0].zero)
    // }


    caps
    // master_cap_sheet





    // if let Err(e) = write_to_file(result2, "res.csv") {
    //     eprintln!("{}", e);
    //     process::exit(1);
    // } else {
    //     println!("Inner Join Complete");
    // }

}

// fn make_team_objects() {

// }

// re-signing vs free agents bird rights
pub fn signing() {
    // can we sign the player with space?
    // if not try to use exceptions
}
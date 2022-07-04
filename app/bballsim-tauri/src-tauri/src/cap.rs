

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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    Team: String
    

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

use rusty_money::{Money, Round, iso};


// this whole thing should really be in team objects
// make an hashmap of team objects -> write this to json and read from json
// all relevant info readily available
pub fn trade(team1: String, team2: String, players1: Vec<Player>, players2: Vec<Player>) {
    let master_cap_sheet: HashMap<String, Vec<Player>> = get_cap_dict();


    let team_1_books = &master_cap_sheet[&team1];
    let team_2_books = &master_cap_sheet[&team2];

    // let ind = team_1_books.index_of()

    //find the players in master_cap_sheet
    //swap them


    // THIS NEEDS A LOT OF LOGIC TO UNDERSTAND CORRECT NUMBER
    // NEED TO WORK OUT WHAT NON GUARANTEES THERE ARE
    // find out rules for what counts towards cap
    let mut salary_1 = Money::from_minor(0, iso::USD);
    for player in team_1_books{
        if &player.one.len() < &1{
            continue
        }
        salary_1 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }

    println!("{} {}", team1, salary_1);

    let mut salary_2 = Money::from_minor(0, iso::USD);
    for player in team_2_books{
        if &player.one.len() < &1{
            continue
        }
        salary_2 += Money::from_str(&player.one[1..], iso::USD).unwrap()
    }

    println!("{} {}", team2, salary_2);


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
    

}


pub fn main() {
    let brad: Player = Player {
        Player: "Bradley Beal".to_string(),
        Age: 29,
        zero: "$33,724,200".to_string(),
        one: "$37,262,300".to_string(),
        two: "".to_string(),
        three: "".to_string(),
        four: "".to_string(),
        five: "".to_string(),
        Signed: "".to_string(),
        Guaranteed: "".to_string(),
        Team: "Wizards".to_string()
    };
    let russ: Player = Player {
        Player: "Russel Westbrook".to_string(),
        Age: 29,
        zero: "$33,724,200".to_string(),
        one: "$37,262,300".to_string(),
        two: "".to_string(),
        three: "".to_string(),
        four: "".to_string(),
        five: "".to_string(),
        Signed: "".to_string(),
        Guaranteed: "".to_string(),
        Team: "Lakers".to_string()
    };
    let brad2: Player = Player {
        Player: "Bradley Beal".to_string(),
        Age: 29,
        zero: "$33,724,200".to_string(),
        one: "$37,262,300".to_string(),
        two: "".to_string(),
        three: "".to_string(),
        four: "".to_string(),
        five: "".to_string(),
        Signed: "".to_string(),
        Guaranteed: "".to_string(),
        Team: "Wizards".to_string()
    };
    let russ2: Player = Player {
        Player: "Russel Westbrook".to_string(),
        Age: 29,
        zero: "$33,724,200".to_string(),
        one: "$37,262,300".to_string(),
        two: "".to_string(),
        three: "".to_string(),
        four: "".to_string(),
        five: "".to_string(),
        Signed: "".to_string(),
        Guaranteed: "".to_string(),
        Team: "Lakers".to_string()
    };
    trade("Wizards".to_string(), "Heat".to_string(), [brad].to_vec(), [russ].to_vec());
    trade("Bucks".to_string(), "Suns".to_string(), [brad2].to_vec(), [russ2].to_vec())
}

use serde::{Serialize, Deserialize};

pub fn get_cap_dict() -> HashMap<String, Vec<Player>>{
    println!("{}", 1);

    let mut contract_data = match read_from_file("cap_sheet.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    // let mut contract_data: DataSet = read_from_file("contracts.csv");

    // hashmap with cap sheet for each team
    let mut master_cap_sheet: HashMap<String, Vec<Player>> = HashMap::new();
    let mut cur_team: String = "".to_string();
    let mut cur_players: Vec<Player> = Vec::new();
    for row in contract_data.records.iter(){
        // println!("{}", row.as_slice())
        let r: Player = row.deserialize(None).unwrap();
        // let t = serde_json::from_str(row).unwrap();

        cur_players.push(r.clone());
        if cur_team != r.Team {
            if cur_team == "" {
                cur_team = r.Team;
                continue
            }else {
                master_cap_sheet.insert(cur_team, cur_players.clone());
                cur_players.clear();
                cur_team = r.Team;
            }
        }
        
        // use player.team as key for adding to hash
        // println!("{} {} {}", r.Player, r.one, r.Team);
    }

    // for (i, player_vec) in master_cap_sheet {
    //     println!("{} {} {}", i, player_vec[0].Player, player_vec[0].zero)
    // }

    master_cap_sheet





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
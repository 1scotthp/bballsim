

// 1. LOAD IT FROM JS
// 2. I don't fucking know
// 3. database?
use std::fs;
use std::time::Instant;
use rand::distributions::{Distribution, Uniform};

mod cap;


// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

#[derive(serde::Serialize)]
struct box_score_entry {
  dpm: f32,
  name: String,
  pts: i32,
  fgm: i32,
  fg3m: i32,
  fga: i32,
  fg3a: i32,
  reb: i32,
  ast: i32,
  tov: i32,
  stl: i32,
  sec: i32,
  min: i32
}

// for each play type need a TS / eFG
// struct play_type_effectiveness {
//   play: String,
//   3Pfr: f32,
//   2Pfr: f32,
//   TOVfr: f32,
//   FTr: f32,
// }


// 1. call randomizer with play_type_freq list to get index -- use usize type for indices
// 2. use same index to check play_type_info_arr
// 3. using shot type freq arr call randomizer to get index
// 4. check same index of play_type_effectiveness to get eFG
// 5. apply team transformation to eFG
// 6. get outcome






// this function takes in a standard array and returns the 3 different kinds
// fn gen_freq_arrays () -> {
  
// }

// 0 - 3Pfr
// 1 - 2Pfr
// 2 - TOVfr
// 3 - 
// [[.2, .3, .4, .1], [.2, .3, .4, .1]] //array of frequency arrays


// GIVEN A FREQ ARRAY GENERATE THE 3 ARRAYS 
// ONCE PER GAME
// 0 - iso
// 1 - pnr
// 2 - transition
// [.2, .3, .5] // frequency array < 8 seconds
// [.2, .4, .4] // frequency array > 8 seconds
// [.2, .4, .4] // after offensive rebound



// [25, 35, 34]
//ASSIGN INFO, FREQ ARRAY INDICES TO PLAY TYPES. shd be the same

// THESE ARE CALCULATED ONCE PER GAME
// independent of team factors
struct Player_ratings {
  pace: f32,
  ast: f32,
  dreb: f32, 
  oreb: f32,
  usg: f32,
  threePr: f32,
  three: f32,
  twoPr: f32,
  two: f32,
  FTr: f32,
  FT: f32,
  TOr: f32,
  off_ball: f32,

  def: f32,
  rim_def: f32,
  to_def: f32,
  FTr_def: f32,
  blk: f32,

  //this is actually 3 arrays will change later
  // play_type_freq_list: f32[], //[.3, .2, .3, .1, 0, 0] shd sum to 1
  // play_type_rating: f32[],
  // play_type_info_arr: [f32[]], // subarrays should sum to 1
  // 3Pr_def: f32,
}


// THESE ARE RECALCULATED EACH TIME THERE IS A SUBSTITUTION
struct team_boosts {
  // off_play_type_mod
  // def_play_type_mod
  // 3Pr_def: f32,
  off_ball: f32, // avg of Player off ball minus league avg
  two_def: f32, // boosts efg on 2s
  to_def: f32, //  boosts to likelihood
  FTr_def: f32, // boosts likelihood of ft
}

// THESE ARE RECALCULATED EACH TIME THERE IS A SUBSTITUTION
struct team_ratings {
  oreb: f32,
  dreb: f32,
  pace: f32,
}


// THESE ARE CONSTANT
struct league_rates {
  oreb: f32,
  two_def: f32,
  to_def: f32,
  FTr_def: f32,
}

// having chris paul on your team makes mikal bridges turn the ball over less
// we have to have a lot of these better if they're not to big
// the bballgm version of this class has only 8 fields all integers or decimals



fn outcome_from_prob (prob: f32) -> bool {
  let num = rand::thread_rng().gen_range(0..100) as f32;
  return prob > num
}

// import { readTextFile } from "@tauri-apps/api/fs";

use std::fs::File;
use priority_queue::PriorityQueue;








// PRIORITY QUEUE

// something with penalty for MP, overall rating, 
fn damn_should_i_sub(on_court: &[&Player], box_score: &HashMap<String, box_score_entry>) -> usize{

  // iterate over all Players on both teams, -> give them a court score. swap by name?

  // let mut court_scores: HashMap<&String, f32> = HashMap::new();
  let mut ratings: PriorityQueue<usize, i32> = PriorityQueue::new();


  // - (box_score[&Player.Player].sec / 20 ) as f32 

  let mut i = 0;
  for Player in on_court {
    // println!("{} {}", Player.Player, (- Player.dpm as f32 - Player.usg / 2.0 as f32) as i32 + 20 + box_score[&Player.Player].sec);

    ratings.push(i, ((Player.dpm * -2.0 - Player.usg) as i32 + i32::pow(box_score[&Player.Player].sec / 60, 2))/3 as i32);

    i+=1
  }

  // // Player who might be subbed is at the top
  // println!("{} {}", on_court[*ratings.peek().unwrap().0], ratings.peek().unwrap().1);

  let ind = ratings.peek().unwrap().0;

  *ind

  // for (key, val) in box_score_home {
  //   // println!("{}", Player);
  //   // court_scores.insert(key, val.sec as f32);
  //   home_on_court.push(key, 20 - val.sec/60 + val.pts);
  // }

  // for (key, val) in box_score_away {
  //   // println!("{}", Player);
  //   // court_scores.insert(key, val.sec as f32);
  //   away_on_court.push(key, 20 - val.sec/60 + val.pts);
  // }



  // assert!(pq.is_empty());
  // pq.push("Apples", 5);
  // pq.push("Bananas", 8);
  // pq.push("Strawberries", 23);

  // for(key, val) in court_scores {
  //   println!("{} {}", key, val)
  // }

  // let mut i = 0;
  // for (key, val) in home_on_court.into_sorted_iter() {
  //   println!("here {} {}", key, val);
  //   i+=1;
  //   if i == 5{
  //     break
  //   }
  // }

  // println!("{} {} {}", box_score_home[&home[0].Player].sec, home[1], court_scores[0])

}

// HashMap<String, box_score_entry>




struct Player_score {
  score: f32
}

struct team_score {
  score: f32
}

//(Vec<box_score_entry>, Vec<box_score_entry>)

use std::time::Duration;
use std::thread::sleep;

// mod playtype_data;



use csv::{Reader, StringRecord, Writer, Error};

/// Internal data set to make aggregation simpler
#[derive(Debug)]
struct DataSet {
    /// Header row of CSV file
    headers: StringRecord,

    /// Records from CSV file
    records: Vec<StringRecord>,
}

use tauri::api::path;

/// Reads csv data from a file and returns a DataSet
fn read_from_file(path: &str) -> Result<DataSet, Box<Error>> {
  let mut reader = Reader::from_path(path)?;

  let headers = reader.headers()?.clone();

  let records = reader
      .records()
      .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

  Ok(DataSet { headers, records })
}


pub struct PackageInfo {
  pub name: String,
  pub version: String,
}

use std::env;
use std::io;
use std::path::PathBuf;

fn inner_main() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;// big?
    // dir.pop();
    // dir.push("Config");
    // dir.push("test.txt");
    Ok(dir)
}


use std::path::Path;
use resolve_path::PathResolveExt;

use static_files;




use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;

use std::fs::OpenOptions;
use std::io::prelude::*;
fn add_task(content: String) {
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("../tasks.txt")
    .expect("Error while opening the tasks file");
  writeln!(file, "{}", content).expect("Error while writing in the tasks file");
}


fn run_game(team1: String, team2: String, data: &String) -> (Vec<box_score_entry>, Vec<box_score_entry>){

  // add_task("Hi".to_string());
  // let paths = fs::read_dir("./").unwrap();

  // for path in paths {
  //     println!("Name: {}", path.unwrap().path().display());
  // }
  // sleep(std::time::Duration::from_secs(5));

  // let data = fs::read_to_string(path).unwrap();
  let d: HashMap<String, Player> = serde_json::from_str(&data).unwrap();

  let mut play_freq: HashMap<String, [f32; 4]> = HashMap::new();
  let mut play_rating: HashMap<String, [f32; 4]> = HashMap::new();


  // here construct 2 hashmaps from d
  // 1. player name to frequency array
  // 2. player name to rating array
  for (i, player) in d.iter() {

    let freq = [player.iso_freq, player.transition_freq, player.pnr_freq, player.pnr_roller_freq];
    let rating = [player.iso_per, player.transition_per, player.pnr_per, player.pnr_roller_per];
    play_freq.insert(player.Player.clone(), freq);
    play_rating.insert(player.Player.clone(), rating);
  }


  // hashmap with all the Players on the 2 teams playing each other
  // maybe add box score stuff to this also
  // let mut PlayersInGame: HashMap<String, &Player> = HashMap::new();


  let mut home: Vec<&Player> = Vec::new();
  let mut away: Vec<&Player> = Vec::new();
  // let mut play_freq_game: HashMap<String, [f32; 4]> = HashMap::new();
  // let mut play_rating_game: HashMap<String, [f32; 4]> = HashMap::new();

  for (i, Player) in d.iter(){
    if Player.team == team1 {
      home.push(Player);

    } else if Player.team == team2 {
      away.push(Player);
    }
  }

  home.sort_by(|a, b| a.dpm.partial_cmp(&b.dpm).unwrap().reverse());
  away.sort_by(|a, b| a.dpm.partial_cmp(&b.dpm).unwrap().reverse());


  let mut box_score_home: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  let mut box_score_away: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  
  for Player in &home {
    let a = Player.Player.clone();
    box_score_home.insert(a.clone(), {box_score_entry {
      dpm: Player.dpm,
      name: a,
      pts: 0,
      fgm: 0,
      fg3m: 0,
      fga: 0,
      fg3a: 0,
      reb: 0,
      ast: 0,
      stl: 0,
      tov: 0,
      sec: 0,
      min: 0
    }});
  }

  for Player in &away {
    let a = Player.Player.clone();

    box_score_away.insert(a.clone(), {box_score_entry{
      dpm: Player.dpm,
      name: a, 
      pts: 0,
      fgm: 0,
      fg3m: 0,
      fga: 0,
      fg3a: 0,
      reb: 0,
      ast: 0,  
      stl: 0,
      tov: 0,
      sec: 0,
      min: 0
    }});
    
    // box_score_away.push(box_score_entry {
    //   name: a,
    //   pts: 0
    // })
  }


  // println!("{}", away[0]);

  // let (court, bench) = home.split_array_mut::<5>();

  let mut timeleft = 720;
  let mut quarter = 1;

  let mut possession: i8 = 0;
  let mut possession_time: i32 = 0;
  let mut outcome: Outcome;
  let mut Player: usize;

  let start = Instant::now();
  //ratings for each Player should be processed into what is useable here

  while quarter < 5 {
    while timeleft > 0 {
      let home_on_court = &home[0 .. 5];
      let away_on_court =  &away[0 .. 5];



      if possession == 0 {
        (possession_time, outcome) = start_possession(&home_on_court, &away_on_court, &play_freq, &play_rating);

        let Player_name = &home_on_court[outcome.Player_index].Player;

        if outcome.shot_type == 2 {
          if outcome.points > 0{
            if let Some(x) = box_score_home.get_mut(Player_name){
              x.fga += 1;
              x.fgm += 1

            }
          } else {
            if let Some(x) = box_score_home.get_mut(Player_name){
              x.fga += 1;
            }

          }
        } else if outcome.shot_type == 3 {
          if outcome.points > 0{
            if let Some(x) = box_score_home.get_mut(Player_name){
              x.fg3a += 1;
              x.fg3m += 1;
              x.fgm += 1;
              x.fga += 1;

            }
          } else {
            if let Some(x) = box_score_home.get_mut(Player_name){
              x.fg3a += 1;
              x.fga += 1
            }

          }
        } else if outcome.shot_type == 0 {
          if let Some(x) = box_score_home.get_mut(Player_name){
            x.tov += 1;
          }

          if let Some(x) = box_score_away.get_mut(Player_name){
            x.stl += 1;
          }

        }
        // let entry = box_score_home.get_mut(Player_name);

        if let Some(x) = box_score_home.get_mut(Player_name){
          x.pts += outcome.points;
        }


        // better way to do this is only when there is a sub, incremement minutes
        for Player in home_on_court{
          if let Some(x) = box_score_home.get_mut(&Player.Player){
            x.sec += possession_time
          }
        }

        for Player in away_on_court{
          if let Some(x) = box_score_away.get_mut(&Player.Player){
            x.sec += possession_time
          }
        }

        // entry.pts += 3;

        // println!("Player {} {}", home_on_court[Player], box_score_home[Player_name].pts);
        // box_score_home[Player].pts += 3;
        
        possession = 1;
      }
      else {
        (possession_time, outcome) = start_possession(&home_on_court, &away_on_court, &play_freq, &play_rating);


        let Player_name = &away_on_court[outcome.Player_index].Player;


        if outcome.shot_type == 2 {
          if outcome.points > 0{
            if let Some(x) = box_score_away.get_mut(Player_name){
              x.fga += 1;
              x.fgm += 1
            }
          } else {
            if let Some(x) = box_score_away.get_mut(Player_name){
              x.fga += 1;
            }

          }

 
        } else if outcome.shot_type == 3 {
          if outcome.points > 0{
            if let Some(x) = box_score_away.get_mut(Player_name){
              x.fg3a += 1;
              x.fg3m += 1;
              x.fgm += 1;
              x.fga += 1;
            }
          } else {
            if let Some(x) = box_score_away.get_mut(Player_name){
              x.fg3a += 1;
              x.fga += 1;
            }

          }
        } else if outcome.shot_type == 0 {
          if let Some(x) = box_score_away.get_mut(Player_name){
            x.tov += 1;
          }

          if let Some(x) = box_score_home.get_mut(Player_name){
            x.stl += 1;
          }

        }
        if let Some(x) = box_score_away.get_mut(Player_name){
          x.pts += outcome.points;
        }

        for Player in home_on_court{
          if let Some(x) = box_score_home.get_mut(&Player.Player){
            x.sec += possession_time
          }
        }

        for Player in away_on_court{
          if let Some(x) = box_score_away.get_mut(&Player.Player){
            x.sec += possession_time
          }
        }

        // box_score_away[Player_name].pts += 3;

        // box_score_away[Player].pts += 3;
        possession = 0
      }
      // should subtract possession time
      timeleft -= possession_time;

      if outcome_from_prob(6.0){
        let i1: usize = damn_should_i_sub(&home_on_court, &box_score_home);
        let mut i2;
        if outcome_from_prob(33.0){
          i2 = 6
        } else if outcome_from_prob(33.0) {
          i2 = 7
        } else  if outcome_from_prob(33.0){
          i2 = 8 
        } else {  
          i2 = 9
        }
        // println!("{} {}", i1, i2);
        home.swap(i1, i2)

        // here update the team ratings based on the substitution
      }

      if outcome_from_prob(6.0){
        let i1: usize = damn_should_i_sub(&away_on_court, &box_score_away);
        let mut i2;
        if outcome_from_prob(40.0){
          i2 = 6
        } else if outcome_from_prob(40.0) {
          i2 = 7
        } else {
          i2 = 8
        }
        // println!("{} {}", i1, i2);
        away.swap(i1, i2)

        // here update the team ratings based on the substitution
      }


    }
    quarter += 1;
    timeleft = 720
  }

  // for (key, val) in &box_score_home {
  //   println!("key: {key} val: {}", val.pts);
  // }

  let duration = start.elapsed();

  println!("Time to simulate 1/2 season of games is: {:?}", duration * 82 * 8);



  // println!("khris {}", box_score_home["Khris Middleton"].pts);

  
  // print_type_of(f.unwrap());
  let mut file = File::create("test.txt");
  // serde_json::to_writer(file.unwrap(), &box_score_home);

  println!("I was invoked from JS!");

  let home_score: Vec<box_score_entry> = box_score_home.into_iter()
                        .map(|(_id, score)| score)
                        .collect();

  let away_score: Vec<box_score_entry> = box_score_away.into_iter()
                                        .map(|(_id, score)| score)
                                        .collect();

  // Ok("worked".into())


  // println!("{}", all_scores[0].name);
  (home_score, away_score)
}



// use tauri::{api::path::{BaseDirectory}, Manager};

use tauri::Manager;
#[tauri::command]
fn my_custom_command(team1: String, team2: String, data: String) -> (Vec<box_score_entry>, Vec<box_score_entry>){
  // cap::main();
  // playtype_data::gen_data();\


  println!("{}", data);
  // let mut m: String = "./darko.json".to_string();

  // let d = tauri::Builder::default().setup(|app: tauri::App| -> Result<String, Error> {
  //   // println!("{}", app);
  //   let path = resolve_path(
  //     &app.config(),
  //     app.package_info(),
  //     &app.env(),
  //     "darko.json",
  //     Some(BaseDirectory::Resource)
  //   );
  //   let data = fs::read_to_string(path.unwrap()).unwrap();
  //   println!("{}", data);
  //   m = data;
  //   // data
  //   // return run_game(team1, team2, &data);
  //   // assert_eq!(path.to_str().unwrap(), "/home/${whoami}/.config/path/to/something");
  //   // Ok(data)
  //   Ok(data)
  // });

  // println!("{}", d);

  return run_game(team1, team2, &data);
  

  // println!("{}", "COMMAND22".to_string());

//   pub struct PackageInfo {
//     pub name: String,
//     pub version: String,
// };

//   let p: PackageInfo = PackageInfo {
//     name: "app".to_string(),
//     version: "0.1.0".to_string()
//   };




  // sleep(Duration(5));
  
  // this needs to come from an api I think
  // maybe write it to a file
  // println!("{}", tauri::api::path::resource_dir(&p, "MACOS").unwrap().display());


  //  let a = "~/darko.json".resolve_in(BaseDirectory::Resource);
  //  let path = resolve_path("resources/darko.json", Some(BaseDirectory::Config)).expect("failed to resolve path");
  //  println!("{}", path);
  // println!("{}", path::resource_dir());

  // sleep(5);

  
  // let data = readTextFile("darko.json")

  


  // let time = Duration::from_secs(5);
  //  sleep(time);
    // println!("{}", data);


  // fs::write("f.json", data).expect("Unable to write file");

    // let iso = read_from_file("master.csv");
    // for row in &iso {
    //     println!("{}", row)
    //     // playtype_dict.insert(row.Player, Vec::new().push(row.rating))
    // }


  
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use tauri::{api::path::{BaseDirectory, resolve_path}, Env};
// async fn main() -> std::io::Result<()> {

// }
// #[actix_web::main]
fn main() {

  
  let generated = generate();

  // for (a, b) in generated.iter(){
  //   println!("yuyuyu {}", a);
  //   let data = fs::read_to_string(a).unwrap();
  //   println!("{}", data)
  // }

  let context = tauri::generate_context!();
  let path = resolve_path(
    context.config(),
    context.package_info(),
    &Env::default(),
    "darko.json",
    Some(BaseDirectory::Resource))
  .expect("failed to resolve path");

  println!("{}", path.display());

  let data = fs::read_to_string(path).unwrap();

  // println!("{}", data);


  // println!("{}", generated);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, cap::get_team_cap_sheet, cap::trade])
    .run(context)
    .expect("error while running tauri application");



  //   HttpServer::new(move || {
  //     let generated = generate();
  //     App::new().service(ResourceFiles::new("/", generated))
  // })
  // .bind("127.0.0.1:8080")?
  // .run()
  // .await
}

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::fmt::Display;
use rand::Rng;


#[derive(Serialize, Deserialize)]
struct Dictionary {
    #[serde(flatten)]
    inner: HashMap<String, Player>,
}

fn get_possession_length(home_on_court:&[&Player], away_on_court:&[&Player]) -> i32 {
  // let mut rng = rand::thread_rng();
  let mut home_pace = 0.0;
  let mut away_pace = 0.0;
  for o in home_on_court {
    home_pace += o.stl_100
  }
  for o in away_on_court {
    away_pace += o.stl_100
  }


  // full league possession distribution
  // adjust for pace 
  // // adjust for how previous possession ended
  let a: f32 = rand::thread_rng().gen_range(2..24) as f32;

  return (home_pace - away_pace + a) as i32;
}

fn sim_turnover(home_on_court: &[&Player], away_on_court: &[&Player]) -> (bool, usize) {

  let o_turnover: f32 = home_on_court.iter().map(|Player| Player.tov_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  let d_turnover: f32 = away_on_court.iter().map(|Player| Player.stl_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  
  let turnover_odds: f32 = (o_turnover + d_turnover) / 1.2;
  let tov = outcome_from_prob(turnover_odds);


  (tov, rand::thread_rng().gen_range(0..4) as usize)
}


fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

use weighted_rand::builder::*;

// println!("{:?}", result);

fn get_index_from_stat(odds_vec: [f32; 5]) -> usize {
  // for odds in odds_vec{
  //   println!("odds {}", odds)
  // }

  let sum: f32 = odds_vec.iter().sum();


  let cheating_coin = ["0","1", "2", "3", "4"];
  let index_weights = odds_vec;

  // println!("odds length {}  index length {}", type(odds_vec), index_weights.len(), odds_vec[0]);

  // print_type_of(&odds_vec);
  // print_type_of(&index_weights);

  let builder = WalkerTableBuilder::new(&index_weights);
  let wa_table = builder.build();

  // If you want to process something in a large number of
  // loops, we recommend using the next_rng method with an
  // external ThreadRng instance.
  let mut result = [""; 10];
  let mut rng = rand::thread_rng();
  for r in &mut result {
      let j = wa_table.next_rng(&mut rng);
      *r = cheating_coin[j];
  }

  return result[0].parse::<usize>().unwrap();

}

fn outcome_from_arr(probs: Vec<f32>) -> usize{
  let between = Uniform::from(0..100);
  let mut rng = rand::thread_rng();
  let mut sum = 0;
  let num = between.sample(&mut rng);
  let mut index = 0;

  for i in probs {
      sum += i as i32;
      if sum > num{
        return index
      }
      index+=1 
  }
  return index - 1
  // println!("{}", sum);

}


// mostly working for getting Player name
fn get_Player(offensive_Players: &[&Player]) -> usize {

  // let mut odds_vec: [f32; 5] = [0.0,0.0,0.0,0.0,0.0];
  // for Player in rangeoffensive_Players{
  //   odds_vec.push(Player.usg)
  // }
  let mut odds_vec: [f32; 5] = [0.0,0.0,0.0,0.0,0.0];
  let mut i = 0;
  while i < 5{
    odds_vec[i] = offensive_Players[i].usg;
    i+=1
  }

  return get_index_from_stat(odds_vec)

}


fn get_shot_type(fga: f32, fg3a: f32) -> i32 {
  let odds_of_3 = fg3a / fga;

  if outcome_from_prob(odds_of_3 * 100.0){
    return 3
  } else {
    return 2
  }
}


struct Outcome {
  Player_index: usize,
  shot_type: i32,
  points: i32,
  // possession_time: i32,
}



fn half_court_possession(offense: &[&Player], defense: &[&Player], play_freq: &HashMap<String, [f32; 4]>, play_rating: &HashMap<String, [f32; 4]>) -> Outcome {

  
  let mut to: bool;
  let mut to_Player_ind: usize; 


  // for (key, value) in play_freq.iter(){
  //   println!("{} {}", key, value[0])
  // }


  // (to, to_Player_ind) = sim_turnover(offense, defense);
  // if to {
  //   return Outcome {
  //     Player_index: to_Player_ind,
  //     shot_type: 0,
  //     points: 0,
  //     // possession_time: 0
  //   }
  // }

  //get the player's index
  let ind: usize = get_Player(offense);

  //get play frequency
  let f = play_freq.get(&offense[ind].Player).unwrap();

  //get play index 0 is iso etc
  let play_index = outcome_from_arr(f.to_vec());
  
  //get players rating array
  // println!("HIHI");
  // for (i, r) in play_rating {
  //   println!("HIHI {} {}", i, r[0]);
  // }
  let rating = play_rating.get(&offense[ind].Player).unwrap()[play_index];

  // println!("{} {} {}", &offense[ind].Player, rating, play_index);
  // println!("{} {} {}", &offense[ind].Player, rating, play_index);

  let shot_type = get_shot_type(offense[ind].fga_100, offense[ind].fg3a_100);

  let mut points = 0;

  if shot_type == 2 {
    if outcome_from_prob(offense[ind].fg2 + rating/10.0 + 5.0){
      points = 2
    }
  } else {
    if outcome_from_prob(offense[ind].fg3 + rating/10.0 + 5.0){
      points = 3
    }
  }



  // println!(" shot {}", points);

  return Outcome {
    Player_index: ind,
    shot_type: shot_type,
    points: points,
    // possession_time: 0
  }



  // print!("{}", offense[ind].fg2);

  // let a = outcome_from_prob(offense[ind].fg2);
  // if a {
  //   return (ind, 2)
  // } else {
  //   return (ind, 0)
  // }

}


// use team reb ratings and league avg to gen outcome
// gen Player based on reb/100
fn sim_rebound(home_on_court: &[&Player], away_on_court: &[&Player], shot_type: i32) -> (bool, i32) {
  let o_rebound: f32 = home_on_court.iter().map(|Player| Player.reb_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  let d_rebound: f32 = away_on_court.iter().map(|Player| Player.reb_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  let d_rebound_odds = 75.0 + o_rebound - d_rebound;
  let d_reb = outcome_from_prob(d_rebound_odds);

  (d_reb, 2)
}



fn start_possession(offense: &[&Player], defense: &[&Player], play_freq: &HashMap<String, [f32; 4]>, play_rating: &HashMap<String, [f32; 4]>) -> (i32, Outcome) {
  // println!("{}", "HERE");
  let mut possession_time = get_possession_length(offense, defense);

  // println!("{}", offense[1]);

  let mut outcome: Outcome = half_court_possession(offense, defense, play_freq, play_rating);

  // if outcome.shot_type == 2 || outcome.shot_type == 3{
  //   sim_rebound(home_on_court, away_on_court, shot_type);
  // }

  // need to be able to have multiple outcomes in the case that theres offensive rebounds

  // outcome.possession_time = possession_time

  (possession_time, outcome)
  // (possession_time, outcome.Player_index, outcome.shot_type)
}

// how to print dictionary
impl std::fmt::Display for Dictionary {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // print!("{}", self.inner)
    for (s, Player) in &self.inner {
      println!("{}", &Player);
    }
    // println!("{}", self.inner);
    write!(f, "{}", "A")
}
}




// impl std::fmt::Display for HashMap<K, Player> {
  
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     // print!("{}", self.inner)
//     for (s, Player) in &self.inner {
//       println!("{}", &Player);
//     }
//     // println!("{}", self.inner);
//     write!(f, "{}", "A")
// }

// }

// fn get_real_Players(team1: &str, team2: &str) -> HashMap<String, &Player> {

  

//   // for (i, Player) in d.iter(){
//   //   if Player.team == team2 {
//   //     // println!("{}", Player.team);
//   //     away.insert(Player.Player.to_string(), Player);
//   //   }
//   // }

//   // println!("{}", home);
//   return home


//   // need to get correct Players from the dictionary


//   // println!("{:?}", serde_json::to_string(&a));
// }
  



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
  #[serde(rename = "Team")]
  pub team: String,
  #[serde(rename = "Player")]
  pub Player: String,
  #[serde(rename = "Experience")]
  pub experience: String,
  #[serde(rename = "DPM")]
  pub dpm: f32,
  #[serde(rename = "DPM Improvement")]
  pub dpm_improvement: String,
  #[serde(rename = "O-DPM")]
  pub o_dpm: String,
  #[serde(rename = "D-DPM")]
  pub d_dpm: String,
  #[serde(rename = "Box DPM")]
  pub box_dpm: String,
  #[serde(rename = "Box O-DPM")]
  pub box_o_dpm: String,
  #[serde(rename = "Box D-DPM")]
  pub box_d_dpm: String,
  #[serde(rename = "FGA/100")]
  pub fga_100: f32,
  #[serde(rename = "FG2%")]
  pub fg2: f32,
  #[serde(rename = "FG3A/100")]
  pub fg3a_100: f32,
  #[serde(rename = "FG3%")]
  pub fg3: f32,
  #[serde(rename = "FG3ARate%")]
  pub fg3arate: String,
  #[serde(rename = "FTA/100")]
  pub fta_100: String,
  #[serde(rename = "FT%")]
  pub ft: String,
  #[serde(rename = "FTARate%")]
  pub ftarate: String,
  #[serde(rename = "Usg%")]
  pub usg: f32,
  #[serde(rename = "Reb/100")]
  pub reb_100: f32,
  #[serde(rename = "Ast/100")]
  pub ast_100: String,
  #[serde(rename = "Ast%")]
  pub ast: String,
  #[serde(rename = "Blk/100")]
  pub blk_100: f32,
  #[serde(rename = "Blk%")]
  pub blk: String,
  #[serde(rename = "Stl/100")]
  pub stl_100: f32,
  #[serde(rename = "Stl%")]
  pub stl: String,
  #[serde(rename = "Tov/100")]
  pub tov_100: f32,
  #[serde(rename = "iso_FREQ")]
  pub iso_freq: f32,
  #[serde(rename = "iso_PERCENTILE")]
  pub iso_per: f32,
  #[serde(rename = "transition_FREQ")]
  pub transition_freq: f32,
  #[serde(rename = "transition_PERCENTILE")]
  pub transition_per: f32,
  #[serde(rename = "pnr_handler_FREQ")]
  pub pnr_freq: f32,
  #[serde(rename = "pnr_handler_PERCENTILE")]
  pub pnr_per: f32,
  #[serde(rename = "pnr_roller_FREQ")]
  pub pnr_roller_freq: f32,
  #[serde(rename = "pnr_roller_PERCENTILE")]
  pub pnr_roller_per: f32,
  
}



// how to print Player
impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // println!("HI");
    write!(f, "{}", self.Player)
    // write!(f, "{}", self.team);
}
}

// struct Player {
//   DPM: f64,
//   O-DPM: f64,
//   D-DPM: f64,
  
// }







use std::fs;
use std::time::Instant;


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




fn outcome_from_prob (prob: f32) -> bool {
  let num = rand::thread_rng().gen_range(0..100) as f32;

  return prob > num
  // println!("{}", num);
}


use std::fs::File;
use priority_queue::PriorityQueue;




// PRIORITY QUEUE
fn damn_should_i_sub(on_court: &[&Player], box_score: &HashMap<String, box_score_entry>) -> usize{

  // iterate over all players on both teams, -> give them a court score. swap by name?

  // let mut court_scores: HashMap<&String, f32> = HashMap::new();
  let mut ratings: PriorityQueue<usize, i32> = PriorityQueue::new();


  // - (box_score[&player.player].sec / 20 ) as f32 

  let mut i = 0;
  for player in on_court {
    // println!("{} {}", player.player, (- player.dpm as f32 - player.usg / 2.0 as f32) as i32 + 20 + box_score[&player.player].sec);

    ratings.push(i, ((player.dpm * -2.0 - player.usg) as i32 + i32::pow(box_score[&player.player].sec / 60, 2))/3 as i32);

    i+=1
  }

  // // player who might be subbed is at the top
  // println!("{} {}", on_court[*ratings.peek().unwrap().0], ratings.peek().unwrap().1);

  let ind = ratings.peek().unwrap().0;

  *ind

  // for (key, val) in box_score_home {
  //   // println!("{}", player);
  //   // court_scores.insert(key, val.sec as f32);
  //   home_on_court.push(key, 20 - val.sec/60 + val.pts);
  // }

  // for (key, val) in box_score_away {
  //   // println!("{}", player);
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

  // println!("{} {} {}", box_score_home[&home[0].player].sec, home[1], court_scores[0])

}

// HashMap<String, box_score_entry>


struct player_score {
  score: f32
}

struct team_score {
  score: f32
}

//(Vec<box_score_entry>, Vec<box_score_entry>)


#[tauri::command]
fn my_custom_command(team1: String, team2: String) -> (Vec<box_score_entry>, Vec<box_score_entry>){

  

  println!("{}", "COMMAND22".to_string());


  let data = fs::read_to_string("assets/darko.json").unwrap();
    // println!("{}", data);


  // fs::write("f.json", data).expect("Unable to write file");


  let d: HashMap<String, Player> = serde_json::from_str(&data).unwrap();

  // print!("{:?}", &d);

  // hashmap with all the players on the 2 teams playing each other
  // maybe add box score stuff to this also
  // let mut playersInGame: HashMap<String, &Player> = HashMap::new();


  let mut home: Vec<&Player> = Vec::new();
  let mut away: Vec<&Player> = Vec::new();
  for (i, player) in d.iter(){
    if player.team == team1 {
      home.push(player);

    } else if player.team == team2 {
      away.push(player);
      println!("{}", away.len())
    }
  }


  home.sort_by(|a, b| a.dpm.partial_cmp(&b.dpm).unwrap().reverse());
  away.sort_by(|a, b| a.dpm.partial_cmp(&b.dpm).unwrap().reverse());




  let mut box_score_home: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  let mut box_score_away: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  
  for player in &home {
    let a = player.player.clone();
    box_score_home.insert(a.clone(), {box_score_entry {
      dpm: player.dpm,
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

  for player in &away {
    let a = player.player.clone();

    box_score_away.insert(a.clone(), {box_score_entry{
      dpm: player.dpm,
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
  let mut player: usize;



  let start = Instant::now();
  //ratings for each player should be processed into what is useable here

  while quarter < 5 {
    while timeleft > 0 {
     
      let home_on_court = &home[0 .. 5];
      let away_on_court =  &away[0 .. 5];

      if possession == 0 {
        (possession_time, outcome) = start_possession(&home_on_court, &away_on_court);

        let player_name = &home_on_court[outcome.player_index].player;

        if outcome.shot_type == 2 {
          if outcome.points > 0{
            if let Some(x) = box_score_home.get_mut(player_name){
              x.fga += 1;
              x.fgm += 1

            }
          } else {
            if let Some(x) = box_score_home.get_mut(player_name){
              x.fga += 1;
            }

          }
        } else if outcome.shot_type == 3 {
          if outcome.points > 0{
            if let Some(x) = box_score_home.get_mut(player_name){
              x.fg3a += 1;
              x.fg3m += 1;
              x.fgm += 1;
              x.fga += 1;

            }
          } else {
            if let Some(x) = box_score_home.get_mut(player_name){
              x.fg3a += 1;
              x.fga += 1
            }

          }
        } else if outcome.shot_type == 0 {
          if let Some(x) = box_score_home.get_mut(player_name){
            x.tov += 1;
          }

          if let Some(x) = box_score_away.get_mut(player_name){
            x.stl += 1;
          }

        }


        // let entry = box_score_home.get_mut(player_name);

        if let Some(x) = box_score_home.get_mut(player_name){
          x.pts += outcome.points;
        }


        // better way to do this is only when there is a sub, incremement minutes
        for player in home_on_court{
          if let Some(x) = box_score_home.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        for player in away_on_court{
          if let Some(x) = box_score_away.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        // entry.pts += 3;

        // println!("player {} {}", home_on_court[player], box_score_home[player_name].pts);
        // box_score_home[player].pts += 3;
        
        possession = 1;
      }
      else {
        (possession_time, outcome) = start_possession(&away_on_court, &home_on_court);


        let player_name = &away_on_court[outcome.player_index].player;


        if outcome.shot_type == 2 {
          if outcome.points > 0{
            if let Some(x) = box_score_away.get_mut(player_name){
              x.fga += 1;
              x.fgm += 1
            }
          } else {
            if let Some(x) = box_score_away.get_mut(player_name){
              x.fga += 1;
            }

          }

 
        } else if outcome.shot_type == 3 {
          if outcome.points > 0{
            if let Some(x) = box_score_away.get_mut(player_name){
              x.fg3a += 1;
              x.fg3m += 1;
              x.fgm += 1;
              x.fga += 1;
            }
          } else {
            if let Some(x) = box_score_away.get_mut(player_name){
              x.fg3a += 1;
              x.fga += 1;
            }

          }
        } else if outcome.shot_type == 0 {
          if let Some(x) = box_score_away.get_mut(player_name){
            x.tov += 1;
          }

          if let Some(x) = box_score_home.get_mut(player_name){
            x.stl += 1;
          }

        }



        if let Some(x) = box_score_away.get_mut(player_name){
          x.pts += outcome.points;
        }

        for player in home_on_court{
          if let Some(x) = box_score_home.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        for player in away_on_court{
          if let Some(x) = box_score_away.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        // box_score_away[player_name].pts += 3;

        // box_score_away[player].pts += 3;
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

  println!("Time elapsed in expensive_function() is: {:?}", duration * 82 * 8);



  // println!("khris {}", box_score_home["Khris Middleton"].pts);

  
  // print_type_of(f.unwrap());
  // let mut file = File::create("test.txt");
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

// extern crate tauri_includedir;
// extern crate phf;

use tauri_includedir;
use phf;
use std::env;
include!(concat!(env!("OUT_DIR"), "/assets/darko.json"));

fn main() {


  tauri::Builder::default()
  // .setup(move |app| {
  //   let script_path = app
  //   .path_resolver()
  //   .resolve_resource("assets/darko.json")
  //   .unwrap()
  //   .to_string_lossy()
  //   .to_string();
  // })
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

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

  let a: f32 = rand::thread_rng().gen_range(9..19) as f32;

  // println!("{}", home_on_court[1].fta_100);
  // println!("{}", away_on_court[1].fta_100);


  return (home_pace - away_pace + a) as i32;
}

fn sim_turnover(home_on_court: &[&Player], away_on_court: &[&Player]) -> (bool, i32) {

  let o_turnover: f32 = home_on_court.iter().map(|player| player.tov_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  let d_turnover: f32 = away_on_court.iter().map(|player| player.stl_100 as f32).collect::<Vec<f32>>().into_iter().sum::<f32>();
  
  let turnover_odds: f32 = (o_turnover + d_turnover) / 1.2;

  let tov = outcome_from_prob(turnover_odds);


  (tov, 2)
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


// mostly working for getting player name
fn get_player(offensive_players: &[&Player]) -> usize {

  // let mut odds_vec: [f32; 5] = [0.0,0.0,0.0,0.0,0.0];
  // for player in rangeoffensive_players{
  //   odds_vec.push(player.usg)
  // }
  let mut odds_vec: [f32; 5] = [0.0,0.0,0.0,0.0,0.0];
  let mut i = 0;
  while i < 5{
    odds_vec[i] = offensive_players[i].usg;
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
  player_index: usize,
  shot_type: i32,
  points: i32,
  // possession_time: i32,
}

fn half_court_possession(offense: &[&Player], defense: &[&Player]) -> Outcome {

  let ind: usize = get_player(offense);
  let mut to: bool;
  let mut to_player_ind: i32; 
  (to, to_player_ind) = sim_turnover(offense, defense);
  if to {
    return Outcome {
      player_index: ind,
      shot_type: 0,
      points: 0,
      // possession_time: 0
    }
  }
  let shot_type = get_shot_type(offense[ind].fga_100, offense[ind].fg3a_100);

  let mut points = 0;

  if shot_type == 2 {
    if outcome_from_prob(offense[ind].fg2){
      points = 2
    }
  } else {
  
    if outcome_from_prob(offense[ind].fg3){
      points = 3
    }
  }

  // println!(" shot {}", points);

  return Outcome {
    player_index: ind,
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



fn start_possession(offense: &[&Player], defense: &[&Player]) -> (i32, Outcome) {
  // println!("{}", "HERE");
  let mut possession_time = get_possession_length(offense, defense);

  // println!("{}", offense[1]);

  let mut outcome: Outcome = half_court_possession(offense, defense);

  // outcome.possession_time = possession_time

  (possession_time, outcome)
  // (possession_time, outcome.player_index, outcome.shot_type)
}

// how to print dictionary
impl std::fmt::Display for Dictionary {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // print!("{}", self.inner)
    for (s, player) in &self.inner {
      println!("{}", &player);
    }
    // println!("{}", self.inner);
    write!(f, "{}", "A")
}
}




// impl std::fmt::Display for HashMap<K, Player> {
  
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     // print!("{}", self.inner)
//     for (s, player) in &self.inner {
//       println!("{}", &player);
//     }
//     // println!("{}", self.inner);
//     write!(f, "{}", "A")
// }

// }

// fn get_real_players(team1: &str, team2: &str) -> HashMap<String, &Player> {

  

//   // for (i, player) in d.iter(){
//   //   if player.team == team2 {
//   //     // println!("{}", player.team);
//   //     away.insert(player.player.to_string(), player);
//   //   }
//   // }

//   // println!("{}", home);
//   return home


//   // need to get correct players from the dictionary


//   // println!("{:?}", serde_json::to_string(&a));
// }
  



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
  #[serde(rename = "Team")]
  pub team: String,
  #[serde(rename = "Player")]
  pub player: String,
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
  pub reb_100: String,
  #[serde(rename = "Ast/100")]
  pub ast_100: String,
  #[serde(rename = "Ast%")]
  pub ast: String,
  #[serde(rename = "Blk/100")]
  pub blk_100: String,
  #[serde(rename = "Blk%")]
  pub blk: String,
  #[serde(rename = "Stl/100")]
  pub stl_100: f32,
  #[serde(rename = "Stl%")]
  pub stl: String,
  #[serde(rename = "Tov/100")]
  pub tov_100: f32,
}



// how to print player
impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // println!("HI");
    write!(f, "{}", self.player)
    // write!(f, "{}", self.team);
}
}

// struct Player {
//   DPM: f64,
//   O-DPM: f64,
//   D-DPM: f64,
  
// }







use std::fs;
use std::time::Instant;


// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

#[derive(serde::Serialize)]
struct box_score_entry {
  pts: i32,
  reb: i32,
  ast: i32,
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


  let mut i = 0;
  for player in on_court {
    ratings.push(i, box_score[&player.player].sec / 60 - 5*box_score[&player.player].pts + 20);
    i+=1
  }

  // player who might be subbed is at the top
  // println!("{} {}", ratings.peek().unwrap());

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


#[tauri::command]
fn my_custom_command() -> Result<String, String> {

  println!("{}", "COMMAND22".to_string());

  let team1 = "Milwaukee Bucks";
  let team2 = "Phoenix Suns";
  
  let data = fs::read_to_string("../../../darko.json").unwrap();
    // println!("{}", data);


  // fs::write("f.json", data).expect("Unable to write file");

  // for row in data{
  //   println!("{}", row);
  // }

  let d: HashMap<String, Player> = serde_json::from_str(&data).unwrap();

  // print!("{:?}", &d);

  // hashmap with all the players on the 2 teams playing each other
  // maybe add box score stuff to this also
  // let mut playersInGame: HashMap<String, &Player> = HashMap::new();


  let mut home: Vec<&Player> = Vec::new();
  let mut away: Vec<&Player> = Vec::new();
  for (i, player) in d.iter(){
    if player.team == team1 {
      // println!("{}", player.team);
      home.push(player);
    } else if player.team == team2 {
      away.push(player)
    }
  }



  let mut box_score_home: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  let mut box_score_away: HashMap<String, box_score_entry> = HashMap::with_capacity(15);
  
  for player in &home {
    let a = player.player.clone();
    box_score_home.insert(a, {box_score_entry {
      pts: 0,
      reb: 0,
      ast: 0,
      sec: 0,
      min: 0
    }});
  }

  for player in &away {
    let a = player.player.clone();

    box_score_away.insert(a, {box_score_entry{
      pts: 0,
      reb: 0,
      ast: 0,
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
    

  // box score editing works in this function !!!!!
  while quarter < 5 {
    while timeleft > 0 {
      let home_on_court = &home[0 .. 5];
      let away_on_court =  &away[0 .. 5];

      if possession == 0 {
        (possession_time, outcome) = start_possession(&home_on_court, &away_on_court);

        let player_name = &home_on_court[outcome.player_index].player;

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
          if let Some(x) = box_score_home.get_mut(&player.player){
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

        if let Some(x) = box_score_away.get_mut(player_name){
          x.pts += outcome.points;
        }

        for player in home_on_court{
          if let Some(x) = box_score_home.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        for player in away_on_court{
          if let Some(x) = box_score_home.get_mut(&player.player){
            x.sec += possession_time
          }
        }

        // box_score_away[player_name].pts += 3;

        // box_score_away[player].pts += 3;
        possession = 0
      }
      // should subtract possession time
      timeleft -= possession_time;

      if outcome_from_prob(5.0){
        println!("{}", "shd i sub".to_string());
        let i1: usize = damn_should_i_sub(&home_on_court, &box_score_home);
        let mut i2;
        if outcome_from_prob(40.0){
          i2 = 6
        } else if outcome_from_prob(40.0) {
          i2 = 7
        } else {
          i2 = 8
        }
        // println!("{} {}", i1, i2);
        home.swap(i1, i2)
      }

      if outcome_from_prob(5.0){
        println!("{}", "shd i sub".to_string());
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
      }


    }
    quarter += 1;
    timeleft = 720
  }

  // for (key, val) in &box_score_home {
  //   println!("key: {key} val: {}", val.pts);
  // }

  let duration = start.elapsed();

  println!("Time elapsed in expensive_function() is: {:?}", duration * 82 * 4);



  // println!("khris {}", box_score_home["Khris Middleton"].pts);

  
  // print_type_of(f.unwrap());
  let mut file = File::create("test.txt");
  serde_json::to_writer(file.unwrap(), &box_score_home);

  println!("I was invoked from JS!");

  Ok("worked".into())
}

fn main() {
  tauri::Builder::default()
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
    home_pace += o.stl_100.parse::<f32>().unwrap()
  }
  for o in away_on_court {
    away_pace += o.stl_100.parse::<f32>().unwrap()
  }

  let a: f32 = rand::thread_rng().gen_range(3..15) as f32;

  // println!("{}", home_on_court[1].fta_100);
  // println!("{}", away_on_court[1].fta_100);

  return (home_pace - away_pace + a) as i32;
}

// fn sim_turnover(home_on_court: &[&Player], away_on_court: &[&Player]){

// }


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

  let shot_type = get_shot_type(offense[ind].fga_100, offense[ind].fg3a_100);

  let mut points = 0;

  if shot_type == 2{
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
  pub dpm: String,
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
  pub stl_100: String,
  #[serde(rename = "Stl%")]
  pub stl: String,
  #[serde(rename = "Tov/100")]
  pub tov_100: String,
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






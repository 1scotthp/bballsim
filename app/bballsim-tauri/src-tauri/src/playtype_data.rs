extern crate csv;

use std::error::Error;
use csv::Reader;

// fn gen_rating() -> i32 {

// }


// FUCK RUST

// #[derive(Deserialize)]
// struct Row {
//     Player,
//     TEAM,
//     GP,
//     POSS,
//     FREQ,
//     PPP,
//     PTS,
//     FGM,
//     FGA,
//     FG%,
//     EFG%,
//     "FT FREQ",
//     "TOV FREQ",
//     "SF FREQ",
//     "AND ONE FREQ",
//     "SCORE FREQ",
//     PERCENTILE
// }

struct Row {
    Player: String,
    freq: f32,
    rating: f32
}

// the goal of this function is an array of ratings for the playtype ratings, and an array for frequencies

// pub fn read_csv(f: String) -> Result<Vec<Row>, Box<dyn Error>> {
//   let mut rdr = Reader::from_path(f);

//   let mut play_data = Vec::new();
//   for result in rdr.unwrap().records() {
//       let record = result?;
//     //   println!("{:?}",  record);
//       let row: Row = Row {
//           Player: record.get(0).unwrap().to_string(),
//           freq: record.get(4).unwrap()[..4].parse::<f32>().unwrap(),
//           rating: record.get(record.len() - 1).unwrap().parse::<f32>().unwrap()
//       };
//     play_data.push(row);
//     // println!("{} {}", row.Player, row.rating);

//     // return a dictionary going from Player name freq, rating

//     // get master dictionary by combining smaller dictionaries



//     //   println!("{} {} {}",  row.Player, row.freq, row.rating);

//   }

//   Ok(play_data)
// //   Ok(())

// }

use std::collections::HashMap;


// let playtype_row {
//     freq: Vec<f32>,
//     rating: Vec<f32>
// }

pub fn gen_data() {
    // calls read_csv with a list of files
    // combines into one dictionary and returns it

    let mut playtype_dict = HashMap::<String, Vec<f32>>::new();
    // for i in 1..5 {
    //     hm.insert(i + i, i * i);
    // }
    // println!("{:?}", hm);

    join();


    // let files = ["pnr.csv", "transition.csv"];

    // // every Player has a frequency array and a rating array

    // let iso = read_csv("iso.csv".to_string()).unwrap();
    // for row in &iso {
    //     println!("{}", row.Player)
    //     // playtype_dict.insert(row.Player, Vec::new().push(row.rating))
    // }

    // for f in &files {
    //     let data = read_csv(f.to_string()).unwrap();
    //     for row in &data {
    //         playtype_dict[&row.Player].push(row.rating)
    //     }

    // }

}


// 
use std::fmt;
use std::process;

use csv::{StringRecord, Writer};

/// A simple error handler structure
#[derive(Debug)]
struct IndexError(String);

// impl fmt::Display for IndexError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Index Error: {}", self.0)
//     }
// }

impl Error for IndexError {}

/// Internal data set to make aggregation simpler
#[derive(Debug)]
struct DataSet {
    /// Header row of CSV file
    headers: StringRecord,

    /// Records from CSV file
    records: Vec<StringRecord>,
}

impl DataSet {
    /// Creates a new data set
    fn new(headers: StringRecord, records: Vec<StringRecord>) -> Self {
        DataSet { headers, records }
    }

    /// Finds the index of a column given the column name
    ///
    /// # Arguments
    ///
    /// * `key` -> The column name
    ///
    /// # Errors
    ///
    /// An error occurs if column name does not exist.
    fn key_index(&self, key: &str) -> Result<usize, Box<Error>> {
        match self.headers.iter().position(|column| column == key) {
            Some(index) => Ok(index),
            None => Err(Box::new(IndexError(format!(
                "Column '{}' does not exist.",
                key
            )))),
        }
    }

    /// Sort data records by the given index.
    ///
    /// # Errors
    ///
    /// An error occurs if the index is out of bounds
    fn sort_by_index(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.headers.len() {
            Err(Box::new(IndexError(format!(
                "Index '{}' out of bounds",
                index
            ))))
        } else {
            self.records.sort_by(|a, b| a[index].cmp(&b[index]));
            Ok(())
        }
    }
}

/// This trait defines aggregation methods for the internal data set
trait Aggregate {
    fn inner_join(&mut self, right: &mut Self, key: &str, index: char) -> Result<DataSet, Box<dyn Error>>;
}

impl Aggregate for DataSet {
    /// Performs an inner join on two data sets, where `self` is the left table.
    ///
    /// # Arguments
    ///
    /// * `right` -> The right data set for the join
    /// * `key` -> The column name to join on
    fn inner_join(&mut self, right: &mut Self, key: &str, index: char) -> Result<DataSet, Box<dyn Error>> {
        // Get column index
        let left_index = self.key_index(key)?;
        let right_index = right.key_index(key)?;

        let mut col_names: Vec<String> = Vec::new();
        for field in right.headers.iter() {
            // if field == "Player" || field == "TEAM"{
            //     continue
            // }
            let mut hello = String::from(field);
            hello.push(index);
            col_names.push(hello);
        }


        // let new_col_titles = right.headers.iter().flat_map(|x| {
        //     x.to_owned().push_str("iso")
            
        // }).collect::<>();
        // Merge headers
        let headers = StringRecord::from(
            self.headers
                .iter()
                .chain(&StringRecord::from(col_names))
                .collect::<Vec<&str>>(),
        );

        let mut records = vec![];

        if self.records.is_empty() || right.records.is_empty() {
            return Ok(DataSet::new(headers, records));
        }

        // Sort data sets by the column index
        // Required to for this sort algorithm
        self.sort_by_index(left_index)?;
        right.sort_by_index(right_index)?;

        let mut left_cursor = 0;
        let mut right_cursor = 0;

        while left_cursor < self.records.len() && right_cursor < right.records.len() {
            // If two fields match, merge fields into a single record
            // and add to records vector
            // If they don't match and the left value is less then right value advance the left cursor
            // else advance the right cursor
            if self.records[left_cursor][left_index] == right.records[right_cursor][right_index] {
                let record = StringRecord::from(
                    self.records[left_cursor]
                        .iter()
                        .chain(right.records[right_cursor].iter())
                        .collect::<Vec<&str>>(),
                );

                records.push(record);

                // Since data sets are sorted
                // Advance cursor through right data set to
                // see if there are matches
                let mut k = right_cursor + 1;
                while k < right.records.len()
                    && self.records[left_cursor][left_index] == right.records[k][right_index]
                {
                    let record = StringRecord::from(
                        self.records[left_cursor]
                            .iter()
                            .chain(right.records[k].iter())
                            .collect::<Vec<&str>>(),
                    );

                    records.push(record);

                    k += 1;
                }

                left_cursor += 1;
            } else if self.records[left_cursor][left_index]
                < right.records[right_cursor][right_index]
            {
                left_cursor += 1;
            } else {
                right_cursor += 1;
            }
        }

        Ok(DataSet::new(headers, records))
    }
}

/// Reads csv data from a file and returns a DataSet
fn read_from_file(path: &str) -> Result<DataSet, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;

    let headers = reader.headers()?.clone();

    let records = reader
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    Ok(DataSet { headers, records })
}

/// Converts given DataSet to CSV and writes to file
fn write_to_file(data: DataSet, path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(path)?;

    writer.write_record(data.headers.iter())?;

    for record in data.records {
        writer.write_record(record.iter())?;
    }

    Ok(())
}

fn join() {
    // Read customers
    let mut iso = match read_from_file("iso.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Read orders
    let mut pnr = match read_from_file("pnr.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut transition = match read_from_file("transition.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Join records
    let mut result = match iso.inner_join(&mut pnr, "Player", '1') {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let result2 = match result.inner_join(&mut transition, "Player", '2') {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Write results to file
    if let Err(e) = write_to_file(result2, "res.csv") {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        println!("Inner Join Complete");
    }
}


import { useEffect, useState } from "react";
import Dropdown from "react-dropdown";
import Papa from "papaparse";
// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";

// Allowed extensions for input file
// const allowedExtensions = ["csv"];

// pull up team rosters based on a dropdown menu
// Player,Age,zero,one,two,three,four,five,Signed,Guaranteed,Team,Position,EXP,TeamAbbr,Amount,option_type,option_type
class player_cap_entry {
  Player: string;
  Age: number;
  zero: String;
  one: String;
  two: String;
  three: String;
  four: String;
  five: String;
  Signed: String;
  Team: String;
  checked: boolean;

  constructor(
    Player: string,
    Age: number,
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    Signed: String,
    Team: String,
    checked: boolean
  ) {
    this.Player = Player;
    this.Age = Age;
    this.zero = zero;
    this.one = one;
    this.two = two;
    this.three = three;
    this.four = four;
    this.five = five;
    this.Signed = Signed;
    this.Team = Team;
    this.checked = checked;
  }
  displayInfo() {
    return this.Player + "is " + this.zero + " years old!";
  }
}

const Trade = () => {

  const read_data = async (fileName: string) => {
    const contents = await readTextFile(fileName, { dir: BaseDirectory.Resource });
    // console.log(typeof(contents))
    // const player_data_obj = JSON.parse(contents)

    return contents
  }

  
  const [awayTeamName, setAwayTeamName] = useState<String>("Milwaukee Bucks");
  const [homeTeamName, setHomeTeamName] = useState<String>("Utah Jazz");
  // State to store parsed data
  //   const [parsedData, setParsedData] = useState<Object[]>();

  //   //State to store table Column name
  //   const [tableRows, setTableRows] = useState<any[]>();

  //   //State to store the values
  //   const [values, setValues] = useState([]);

  //   const [homeChecked, setHomeChecked] = useState<number[]>([]);

  //   console.log(name_arr[name_arr.length - 1], "JHI")

  //   const removeItem = (index: number) => {
  //     setHomeChecked(
  //       homeChecked.filter((_, i) => i !== index)
  //     );
  //   }

  const get_mascot_from_name = (team: String) => {
    let name_arr = team.split(" ");
    return name_arr[name_arr.length - 1];
  };

  const [checkedHome, setCheckedHome] = useState<number[]>([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
  ]);

  const loadHome = (team: string) => {
    read_data("master_cap.json").then((data) => {
      invoke("get_team_cap_sheet", { team: get_mascot_from_name(team), data: data}).then(
        (r: any) => {
            console.log("in", r);
  
          for (let player in r) {
            r[player].checked = false;
          }
          setHomeData(r);
        }
      );

    })
    
  };

  const loadAway = (team: string) => {
    read_data("master_cap.json").then((data) => {
      invoke("get_team_cap_sheet", { team: get_mascot_from_name(team), data: data}).then(
        (r: any) => {
            console.log("in", r);
  
          for (let player in r) {
            r[player].checked = false;
          }
          setAwayData(r);
        }
      );

    })
  };

  useEffect(() => {
      loadAway("Bucks")
      loadHome("Suns")
    // Papa.parse("master_cap.csv", {
    //   header: true,
    //   skipEmptyLines: true,
    //   complete: function (results) {
    //     let rowsArray: Object[] = [];
    //     let valuesArray: Object[] = [];
    //     // Iterating data to get column name and their values
    //     results.data.map((d: any) => {
    //       rowsArray.push(Object.keys(d));
    //       valuesArray.push(Object.values(d));
    //     });
    //     console.log("HI", results.data);
    //     // Parsed Data Response in array format
    //     //   setParsedData(results.data);
    //     // Filtered Column Names
    //     //   setTableRows(rowsArray[0]);
    //     //   // Filtered Values
    //     //   setValues(valuesArray);
    //   },
    // });
    // console.log("here");
  }, []);

  const options = [
    "Phoenix Suns",
    // "Milwaukee Bucks",
    "Dallas Mavericks",
    "Utah Jazz",
    "Memphis Grizzlies",
    "Los Angeles Lakers",
    "Los Angeles Clippers",
  ];
  const defaultOption = options[0];

  //   const [home, setHome] = useState<Player[]>([]);
  //   const [away, setAway] = useState<Player[]>([]);
  const [homeData, setHomeData] = useState<any[]>([]);
  const [awayData, setAwayData] = useState<any[]>([]);

  const [validTrade, setValidTrade] = useState<boolean>(true);
  const [tradeResponse, setTradeResponse] = useState<String>("")
  //   const [checked, setChecked] = useState<boolean[]>([]);

  return (
    <div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-evenly",
        }}
      >
        <Dropdown
          options={options}
          onChange={(name) => {
            setHomeTeamName(name.value);
            loadHome(name.value);
          }}
          value={defaultOption}
          placeholder="Select an option"
        />
        <button
          onClick={() => {
            const homeTrading = homeData.filter((player) => {
              return player.checked;
            });
            const awayTrading = awayData.filter((player) => {
              return player.checked;
            });


            let team1 = get_mascot_from_name(homeTeamName);
            let team2 = get_mascot_from_name(awayTeamName);

            read_data("master_cap.json").then((data) => {
              invoke("trade", {
                team1: team1,
                team2: team2,
                players1: homeTrading,
                players2: awayTrading,
                data: data
              }).then((a: any) => {
                  setValidTrade(a[0])
                  setTradeResponse(a[1])
              });

            })
           
         
          }}
        >
          Trade
        </button>
        <div>
            {validTrade ? <p>GOOD</p> : <p>BAD</p>}
            <p>{tradeResponse}</p>
        </div>
        <Dropdown
          options={options}
          onChange={(name) => {
            setAwayTeamName(name.value);
            loadAway(name.value);
          }}
          value={defaultOption}
          placeholder="Select an option"
        />
      </div>
      <div style={{ display: "flex", flexDirection: "row", width: "50%" }}>
        <table style={{ padding: 20, width: "50%" }}>
            <th>Trade</th>
          <th>Name</th>
          <th>Age</th>
          <th>2021-22</th>
          <th>2022-23</th>
          <th>two</th>
          <th>three</th>
          <th>four</th>
          <th>five</th>
          <th>Signed</th>
          <th> Guaranteed</th>
          <th> Team</th>
          {/* Position</th>
              EXP</th>
              TeamAbbr</th>
              Amount</th>
              option_type</th>
              option_type */}

          {homeData.map((val: player_cap_entry, key) => {
            return (
              <tr key={key}>
                <td>
                  <input
                    type="checkbox"
                    onChange={() => {
                      var data = [...homeData];
                      var index = key;
                      //   var index = data.findIndex(obj => obj. === id);
                      data[index].checked = !data[index].checked;
                      setHomeData(data);
                    }}
                    checked={val.checked}
                  />
                </td>
                <td>{val.Player}</td>
                <td>{val.Age}</td>
                <td>{val.zero}</td>
                <td>{val.one}</td>
                <td>{val.two}</td>
                <td>{validTrade}</td>
                {/* <td>{val.Team}</td> */}

                {/* <td>{Math.round(val.sec / 60)}</td>
                  <td>{Math.round(val.pts)}</td>
                  <td>
                    {val.fgm} - {val.fga}
                  </td>
                  <td>
                    {val.fg3m} - {val.fg3a}
                  </td>
                  <td>{val.tov}</td> */}
              </tr>
            );
          })}
        </table>

        <table style={{ padding: 20 }}>
          <tr>
          <th>Trade</th>
            <th>Name</th>
            <th>Age</th>
            <th>2021-22</th>
            <th>one</th>
            <th>two</th>
            <th>three</th>
            <th>four</th>
            <th>five</th>
            <th>Signed</th>
            <th> Guaranteed</th>
            <th> Team</th>
            {/* Position</th>
              EXP</th>
              TeamAbbr</th>
              Amount</th>
              option_type</th>
              option_type */}
          </tr>
          {awayData.map((val: player_cap_entry, key) => {
            return (
              <tr key={key}>
                <td>
                  <input
                    type="checkbox"
                    onChange={() => {
                      var data = [...awayData];
                      var index = key;
                      //   var index = data.findIndex(obj => obj. === id);
                      data[index].checked = !data[index].checked;
                      setAwayData(data);
                    }}
                    checked={val.checked}
                  />
                </td>
                <td>{val.Player}</td>
                <td>{val.Age}</td>
                <td>{val.zero}</td>
                <td>{val.one}</td>
                <td>{val.two}</td>
                {/* <td>{val.Team}</td> */}

                {/* <td>{Math.round(val.sec / 60)}</td>
                  <td>{Math.round(val.pts)}</td>
                  <td>
                    {val.fgm} - {val.fga}
                  </td>
                  <td>
                    {val.fg3m} - {val.fg3a}
                  </td>
                  <td>{val.tov}</td> */}
              </tr>
            );
          })}
        </table>
      </div>
    </div>
  );
};

export default Trade;

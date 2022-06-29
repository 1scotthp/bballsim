import React, { useState, useEffect, useRef } from "react";

import logo from "./logo.svg";
import "./App.css";
import FlatList from "flatlist-react";
import ReactTable from "react-table";

// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";


import Dropdown from "react-dropdown";
import "react-dropdown/style.css";

import { appDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
// const invoke = window.__TAURI__.invoke

// Invoke the command

// useEffect(() => {

// }, [])

// this object should be a player box score entry type like in rust. need to also include names

class box_score_entry {
  name: string;
  pts: number;
  sec: number;
  fg3m: number;
  fgm: number;
  fga: number;
  fg3a: number;
  stl: number;
  tov: number;

  constructor(
    name: string,
    pts: number,
    sec: number,
    fga: number,
    fg3a: number,
    fg3m: number,
    fgm: number,
    to: number,
    stl: number
  ) {
    this.name = name;
    this.pts = pts;
    this.sec = sec;
    this.fg3m = fg3m;
    this.fgm = fgm;
    this.fga = fga;
    this.fg3a = fg3a;
    this.stl = stl;
    this.tov = to;
  }
  displayInfo() {
    return this.name + "is " + this.pts + " years old!";
  }
}


function App() {

  const appDirPath = appDir().then((appDir) => {
    const filePath = join(appDir, "assets/video.mp4").then((p) => {
      const assetUrl = convertFileSrc(p);
      console.log("PPP");
    });
  });



  const [homeTeamName, setHomeTeamName] = useState<String>("Utah Jazz");
  const [awayTeamName, setawayTeamName] = useState<String>("Milwaukee Bucks");
  const [home, setHome] = useState<box_score_entry[]>([]);
  const [away, setAway] = useState<box_score_entry[]>([]);

  // const [disp, setDisp] = useState<Object[]>();

  // let box_score = useRef<string, Object>(null)

  console.log("HERE");

  // useEffect(() => {
  //   console.log("useeffect");

  //   // alert("HI" + Object.keys(b).length)

  //   // invoke("my_custom_command")
  //   //   .then((message: any) => {
  //   //     setHome(
  //   //       message[0].sort(
  //   //         (a: box_score_entry, b: box_score_entry) => b.sec - a.sec
  //   //       )
  //   //     );
  //   //     setAway(
  //   //       message[1].sort(
  //   //         (a: box_score_entry, b: box_score_entry) => b.sec - a.sec
  //   //       )
  //   //     );

  //   //     alert("HI 2 " + message);

  //   //     // if (Object.keys(b).length !== 0){
  //   //     //   return
  //   //     // }

  //   //     alert(Object.keys(message));

  //   //     // JSON.print(b)

  //   //     console.log("HI", message);
  //   //   })
  //   //   .catch((err) => console.log(err));
  // }, [home, away]);

  // setb(b.sort(function(a, b) {
  // return a.sec - b.sec;
  // }));

  // setb(b.sort((a: box_score_entry, b: box_score_entry) => a.sec > b.sec ? a.sec : b.sec));

  const homeFg = home.map((player) => (
    <p>
      {player.fgm} - {player.fga}
    </p>
  ));

  const homeFg3a = home.map((player) => <p>{player.fg3a}</p>);

  const homePts = home.map((player) => <p>{player.pts}</p>);

  const homeNames = home.map((player) => <p>{player.name}</p>);

  const homeMins = home.map((player) => <p>{Math.round(player.sec / 60)}</p>);

  const awayFga = home.map((player) => <p>{player.fga}</p>);

  const awayPts = away.map((player) => <p>{player.pts}</p>);

  const awayNames = away.map((player) => <p>{player.name}</p>);

  const awayMins = away.map((player) => <p>{Math.round(player.sec / 60)}</p>);

  const divider = away.map((player) => <p>|</p>);

  const runGame = () => {
    console.log("NAMES", homeTeamName, awayTeamName);
    alert(homeTeamName);
    alert(awayTeamName);
    invoke("my_custom_command", {
      team1: homeTeamName,
      team2: awayTeamName,
    });
    // .then((message: any) => {
    //   setHome(
    //     message[0].sort(
    //       (a: box_score_entry, b: box_score_entry) => b.sec - a.sec
    //     )
    //   );
    //   setAway(
    //     message[1].sort(
    //       (a: box_score_entry, b: box_score_entry) => b.sec - a.sec
    //     )
    //   );
    // }).catch((e) => {
    //   console.log(e)
    // });
  };

  const options = [
    "Phoenix Suns",
    // "Milwaukee Bucks",
    "Dallas Mavericks",
    "Utah Jazz",
    "Memphis Grizzlies",
  ];
  const defaultOption = options[0];

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        {/* <FlatList list={b} renderItem={() => {
          <p>HI</p>
        }}>
          </FlatList> */}
      </header>
      {/* <div>
        {Object.keys(b)[0]}
        </div> */}
      <div>
        <Dropdown
          options={options}
          onChange={(name) => {
            console.log("NAME", name);
            setHomeTeamName(name.value);
          }}
          value={defaultOption}
          placeholder="Select an option"
        />
        <button onClick={() => runGame()}>New Game</button>

        <div style={{ display: "flex", flexDirection: "row", padding: 20 }}>
          home{" "}
          {home
            .map((player) => player.pts)
            .reduce((accumulator, current) => {
              return accumulator + current;
            }, 0)}
          <table style={{ padding: 20 }}>
            <tr>
              <th>Name</th>
              <th>Min</th>
              <th>Pts</th>
              <th>FG</th>
              <th>3FG</th>
              <th>TO</th>
            </tr>
            {home.map((val, key) => {
              return (
                <tr key={key}>
                  <td>{val.name}</td>
                  <td>{Math.round(val.sec / 60)}</td>
                  <td>{val.pts}</td>
                  <td>
                    {val.fgm} - {val.fga}
                  </td>
                  <td>
                    {val.fg3m} - {val.fg3a}
                  </td>
                  <td>{val.tov}</td>
                </tr>
              );
            })}
          </table>
          away{" "}
          {away
            .map((player) => player.pts)
            .reduce((accumulator, current) => {
              return accumulator + current;
            }, 0)}
          <table style={{ padding: 20 }}>
            <th>Name</th>
            <th>Min</th>
            <th>Pts</th>
            <th>FG</th>
            <th>3FG</th>
            <th>TO</th>

            {away.map((val, key) => {
              return (
                <tr key={key}>
                  <td>{val.name}</td>
                  <td>{Math.round(val.sec / 60)}</td>
                  <td>{val.pts}</td>
                  <td>
                    {val.fgm} - {val.fga}
                  </td>
                  <td>
                    {val.fg3m} - {val.fg3a}
                  </td>
                  <td>{val.tov}</td>
                </tr>
              );
            })}
          </table>
        </div>

        {/* <div style={{ display: "flex", flexDirection: "row" }}>
  
          <div style={{
            width: "10%",
          }}>
            <p>FGA</p>
            <ul>{homeFg}</ul>
          </div>

          <div style={{
            width: "10%",
          }}>
            <p>FG3A</p>
            <ul>{homeFg3a}</ul>
          </div>
        
          <div style={{
            width: "10%",
          }}>
             <p>PTS</p>
          <ul>{homePts}</ul>
          </div>


          <div style={{
            width: "25%",
          }}>
             <p>Name</p>
          <ul>{homeNames}</ul>
          </div>

          <div style={{
            width: "10%",
          }}>
             <p>Mins</p>
          <ul>{homeMins}</ul>
          </div>

          <ul>{divider}</ul>

          <ul>{awayFga}</ul>
          <ul>{awayPts}</ul>
          <ul>{awayNames}</ul>
          <ul>{awayMins}</ul>
        </div> */}
      </div>
    </div>
  );
}

export default App;

{
  /* <FlatList
           list={Object.entries(b)}
           renderItem={() => {
            <div> hi </div>
           }
           }>
          </FlatList> */
}
{
  /* {Object.keys(b)} */
}

import React, { useState, useEffect, useRef } from "react";

import logo from "./../logo.svg";
import "./App.css";
import FlatList from "flatlist-react";
import ReactTable from "react-table";

// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

import Game from "./Game";
import Trade from "./Trade";

import "react-dropdown/style.css";

import { appDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";

// const RootStack = createStackNavigator<RootStackParamList>();

// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
// const invoke = window.__TAURI__.invoke

// Invoke the command

// useEffect(() => {

// }, [])

// this object should be a Player box score entry type like in rust. need to also include names

// class box_score_entry {
//   name: string;
//   pts: number;
//   sec: number;
//   fg3m: number;
//   fgm: number;
//   fga: number;
//   fg3a: number;
//   stl: number;
//   tov: number;

//   constructor(
//     name: string,
//     pts: number,
//     sec: number,
//     fga: number,
//     fg3a: number,
//     fg3m: number,
//     fgm: number,
//     to: number,
//     stl: number
//   ) {
//     this.name = name;
//     this.pts = pts;
//     this.sec = sec;
//     this.fg3m = fg3m;
//     this.fgm = fgm;
//     this.fga = fga;
//     this.fg3a = fg3a;
//     this.stl = stl;
//     this.tov = to;
//   }
//   displayInfo() {
//     return this.name + "is " + this.pts + " years old!";
//   }
// }

function App() {



  return (
    <div className="App">
      <Router>
        <div>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/trade">Trade</Link>
            </li>
            {/* <li>
          <Link to="/dashboard">Dashboard</Link>
        </li> */}
          </ul>

          <hr />

          {/*
        A <Switch> looks through all its children <Route>
        elements and renders the first one whose path
        matches the current URL. Use a <Switch> any time
        you have multiple routes, but you want only one
        of them to render at a time
      */}
          <Routes>
            <Route path="/" element={<Game />}>

            </Route>
            <Route path="/trade" element={<Trade />}>

            </Route>
            <Route path="/dashboard" element={<Dashboard />}>
      
            </Route>
          </Routes>
        </div>
      </Router>
    </div>
  );
}
function Dashboard() {
  return (
    <div>
      <h2>Dashboard</h2>
    </div>
  );
}

export default App;

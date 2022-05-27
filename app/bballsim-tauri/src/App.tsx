import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';


// With the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
// const invoke = window.__TAURI__.invoke

// Invoke the command



// useEffect(() => {

// }, [])


function App() {




  useEffect(() => {
    
    console.log("useeffect")
    invoke('my_custom_command').then((message: any) => {
      console.log(message)
    })
  }, [])


  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          BALL GM!!!
        </p>
      </header>
    </div>
  );
}

export default App;

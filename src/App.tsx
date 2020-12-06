import React from 'react';
import './App.css';

import logo from './FBW-Logo.svg';

import * as tauri from 'tauri/api/tauri'

function App() {

    function onButton() {
        tauri.invoke({ cmd: 'downloadA32NX'});
    }

    return (
        <div className="App">
            <img src={logo}/>
            <button onClick={onButton}>Download a32nx!</button>
        </div>
    );
}

export default App;

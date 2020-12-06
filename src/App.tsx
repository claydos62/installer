import React from 'react';
import './App.scss';

import logo from './FBW-Tail.svg';

import * as tauri from 'tauri/api/tauri'

function App() {

    function onButton() {
        tauri.invoke({ cmd: 'downloadA32NX'});
    }

    return (
        <div className="App">
            <Logo />
            <TopBar />
            <SideBar />
            <div id="main-content">
                <button onClick={onButton}>Download a32nx!</button>
            </div>
        </div>
    );
}

function Logo() {
    return (
        <div id="fbw-logo-div">
            <img src={logo} alt="FlyByWire Logo" id="fbw-logo"/>
        </div>
    );
}

function TopBar() {
    return (
        <div id="top-bar">
            <p>FlyByWire Installer</p>
        </div>
    )
}

function SideBar() {
    return (
        <div id="side-bar">
            <p>icon1</p>
            <p>icon2</p>
        </div>
    )
}

export default App;

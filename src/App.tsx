import React from 'react';
import * as tauri from 'tauri/api/tauri'

import './App.scss';

import logo from './FBW-Tail.svg';

type MainContentProps = {
    page: string,
}

function App() {
    return (
        <div className="App">
            <Logo />
            <TopBar />
            <SideBar />
            <MainContent page="home" />
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

function MainContent(props: MainContentProps) {
    function onButton() {
        tauri.invoke({ cmd: 'downloadA32NX'});
    }

    switch(props.page) {
        case "home": {
            return (
                <div id="main-content">
                    <button onClick={onButton}>Download a32nx!</button>
                </div>
            )
        }
    }

    return (
        <div />
    )
}

export default App;

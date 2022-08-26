import React from 'react';
import logo from './logo.svg';
import './App.css';
import { GlossaryProvider, Glossary } from "react-glossary";

const myDict = {'doug': {def: 'he is a cool guy'}}

export default class App extends React.Component {

  render(){
    return(
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload.
            <GlossaryProvider value={myDict}>
              This is a story about <Glossary entry='doug'/>
            </GlossaryProvider>
          </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
        </header>
      </div>
    )
  }
}

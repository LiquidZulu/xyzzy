import logo from './logo.svg';
import React from 'react';
import './App.css';
import { glossary } from 'react-glossary';

const context = React.createContext()

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <context.Provider dict={{'doug': {def: 'is a cool guy'}}}>
          This is about <glossary>doug</glossary>.
        </context.Provider>
      </header>
    </div>
  );
}

export default App;

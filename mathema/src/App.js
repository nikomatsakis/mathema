import React, { Component } from 'react';
import ViewCards from "./ViewCards";
import Quiz from "./Quiz";

export default class App extends Component {
  state = {
    subcomponent: null
  }

  componentDidMount() {
  }

  render() {
    if (this.state.subcomponent != null) {
      return this.state.subcomponent;
    }

    let resetApp = () => {
      this.setState({
        subcomponent: null
      });
    };

    let viewCards = () => {
      this.setState({
        subcomponent: <ViewCards resetApp={resetApp}/>
      });
    };

    let startQuiz = () => {
      let duration = parseInt(document.getElementById("duration").value);
      this.setState({
        subcomponent: <Quiz language="gr" duration={duration} resetApp={resetApp}/>
      });
    };

    return (
      <div className="container">
      <div className="col-xs-12">
      <h1>Pick your poison</h1>
      <ul>
        <li> <button onClick={viewCards}>View cards</button> </li>
        <li>
          <button onClick={startQuiz}>Start quiz</button>
          <input type="number" id="duration" name="duration" defaultValue="300"/>
        </li>
      </ul>
      </div>
        </div>
    );
  }

}


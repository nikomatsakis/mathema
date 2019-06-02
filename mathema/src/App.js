import React, { Component } from 'react';
import ViewCards from "./ViewCards";
import Quiz from "./Quiz";

export default class App extends Component {
  state = {
    subcomponent: null
  }

  constructor(props) {
    super(props);
  }

  componentDidMount() {
  }

  render() {
    if (this.state.subcomponent != null) {
      return this.state.subcomponent;
    }

    let viewCards = () => {
      this.setState({
        subcomponent: <ViewCards/>
      });
    };

    let startQuiz = () => {
      this.setState({
        subcomponent: <Quiz language="gr"/>
      });
    };

    return (
      <div className="container">
      <div className="col-xs-12">
      <h1>Pick your poison</h1>
      <ul>
        <li> <button onClick={viewCards}>View cards</button> </li>
        <li> <button onClick={startQuiz}>Start quiz</button> </li>
      </ul>
      </div>
        </div>
    );
  }

}


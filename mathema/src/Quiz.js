import React, { Component } from 'react';

export default class QuizComponent extends Component {
  state = {
    
  }

  componentDidMount() {
    
  }

  render() {
    return (
        <div className="container">
        <div className="col-xs-12">
        </div>
        </div>
    );
  }
}

class QuizCard {
}

function loadWords(lang) {
  const host = "/api";
  let quizCards = await fetch(`${host}/cards`).then(r => r.json());
}

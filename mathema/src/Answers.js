import React, { Component } from 'react';

export default class Answers extends Component {
  render() {
    if (this.props.answers.length > 0) {
      return (
        <div>
        <h3>Answers thus far</h3>
        <ul>
        {this.props.answers.map(([answer, answerIndex], index) => {
          let emoji = (answerIndex !== undefined ? "❤️" : "🤔");
          return (<li key={`answer-${index}`}>{emoji} {answer}</li>);
        })}
        </ul>
          </div>
      );
    } else {
      return null;
    }
  }
}

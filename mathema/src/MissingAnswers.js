import React, { Component } from 'react';

export default class MissingAnswers extends Component {
  render() {
    return (
        <div>
        <h3>Missing Answers</h3>
        <ul>
        {this.props.missingAnswers.map((answer, index) => {
          return (<li key={`missing-answer-${index}`}>{answer}</li>);
        })}
        </ul>
          </div>
    );
  }
}

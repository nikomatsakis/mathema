import React, { Component } from 'react';
import log from 'picolog';
import { HOST } from "./Constants";
import { post } from "./util";

export default class QuizComplete extends Component {
  componentDidMount() {
    this.writeDatabase();
  }

  render() {
    log.debug("rendering quizcomplete");
      return (
          <div className="container">
          <div className="col-xs-12">
          <h1>Quiz complete!</h1>
          <img alt="Office party" src="https://media.giphy.com/media/l0MYt5jPR6QX5pnqM/giphy.gif"/>
          </div>
          </div>
      );
  }

  async writeDatabase() {
    log.debug("writing database");
    await post(`${HOST}/write_db`, "");
  }
}




import React, { Component } from 'react';
import { HOST } from "./Constants";
import Question from "./Question";
import QuizState from "./QuizState";
import Card from "./Card";

export default class QuizComponent extends Component {
  state = {
    questions: null,
    index: 0,
    card: null,
  };

  componentDidMount() {
    this.loadWords();
  }

  updateState(obj) {
    this.setState(Object.assign(this.state, obj));
  }

  async loadWords() {
    let language = this.props.language;

    let questions = await fetch(`${HOST}/quiz_cards/${language}`)
        .then(r => r.json())
        .then(json => {
          return json.map(q => Question.fromJson(q));
        });

    this.updateState({
      questions,
      index: 0,
      card: null,
    });

    this.loadCard();
  }

  async loadCard() {
    let uuid = this.state.questions[this.state.index].uuid;
    console.log("loadCard(): uuid = " + uuid);
    let card = await Card.fetch(uuid);
    this.updateState({ card });
  }

  render() {
    let { questions, index, card } = this.state;

    if (questions == null) {
      return (
          <div className="container">
          <div className="col-xs-12">
          <h1>Loading questions...</h1>
          </div>
          </div>
      );
    }

    // Asked all the cards!
    if (index >= questions.length) {
      return (
          <div className="container">
          <div className="col-xs-12">
          <h1>Quiz complete!</h1>
          <iframe src="https://giphy.com/embed/l0MYt5jPR6QX5pnqM" width="480" height="270" frameBorder="0" class="giphy-embed" allowFullScreen></iframe><p><a href="https://giphy.com/gifs/party-the-office-hard-l0MYt5jPR6QX5pnqM">via GIPHY</a></p>
          </div>
          </div>
      );
    }

    if (card == null) {
      return (
          <div className="container">
          <div className="col-xs-12">
          <h1>Loading card</h1>
          </div>
          </div>
      );
    }

    let question = questions[index];
    let fromLanguage = question.questionKind.fromLanguage();
    let toLanguage = question.questionKind.toLanguage();

    let translateInput = (event) => {
      this.translateInput(event.target);
    };

    return (
        <div className="container">
        <div className="col-xs-12">
        <h1>Translate {card.meanings[fromLanguage]} to {toLanguage}</h1>
        <input type="text" id="name" name="name" size="50" onInput={translateInput}></input>
        </div>
        </div>
    );
  }

  async translateInput(inputElement) {
    let input = inputElement.value;
    let language = this.props.language;
    language = "gr";
    console.log(`input = ${input}`);
    let uri = encodeURI(`${HOST}/transliterate/${language}/${input}`);
    console.log(`uri = ${uri}`);
    let transliterated = await fetch(uri).then(r => r.json());
    inputElement.value = transliterated;
    console.log(`transliterated = ${transliterated}`);
  }
}

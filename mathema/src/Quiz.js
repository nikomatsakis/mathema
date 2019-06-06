import React, { Component } from 'react';
import { HOST } from "./Constants";
import Question from "./Question";
import QuizState from "./QuizState";
import Card from "./Card";

export default class QuizComponent extends Component {
  state = {
    // List of all questions we will ask during the quiz
    questions: null,

    // Current index into the list
    index: 0,

    // Current card we are showing to the user
    card: null,

    // Answers the user has given thus far
    answers: [],

    // Promise for transliteraton: if the user presses a key, we kick off
    // a transliteration request, and then we wait until this
    // promise is resolved to process the next key.
    pendingTransliterations: [],
  };

  componentDidMount() {
    this.loadWords();
  }

  updateState(obj) {
    console.log(`quiz: update state=${JSON.stringify(obj)}`);
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
      answers: [],
    });

    this.loadCard();
  }

  async loadCard() {
    let uuid = this.state.questions[this.state.index].uuid;
    console.log("loadCard(): uuid = " + uuid);
    let card = await Card.fetch(uuid);
    this.updateState({ card });
  }

  // Returns the expected answers for the current question
  // (must be in a state where we *have* a current question).
  expectedAnswers() {
    console.assert(this.state.card != null, "found a null card! state=", JSON.stringify(this.state));
    return this.state.questions[this.state.index].questionKind.expectedAnswers(this.state.card);
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
      let target = event.target;
      let startValue = target.value;
      this.state.pendingTransliterations.push(this.translateInput(target, startValue));
    };

    let submitAnswer = (event) => {
      event.preventDefault();
      let promise =
          Promise.all(this.state.pendingTransliterations).then(this.submitAnswer());
      this.state.pendingTransliterations = [];
    };

    return (
        <div className="container">
        <div className="col-xs-12">
        <h1>Translate {card.meanings[fromLanguage]} to {toLanguage}</h1>
        <ul>
        {this.state.answers.map((answer, index) => (
            <li key={`answer-${index}`}>{answer}</li>
        ))}
      </ul>
        <form onSubmit={submitAnswer}>
        <input type="text" id="input" name="input" size="50" onInput={translateInput}></input>
        </form>
        </div>
        </div>
    );
  }

  async translateInput(inputElement, startValue) {
    // If no data, nothing to transliterate.
    if (startValue.length == 0)
      return;

    // If the input has changed since we were scheduled, just
    // abort. There will be another event scheduled.
    if (inputElement.value != startValue)
      return;

    let language = this.props.language;
    language = "gr"; // temporary
    let uri = `${HOST}/transliterate/${language}/${encodeURIComponent(startValue)}`;
    let transliterated = await fetch(uri).then(r => r.json());
    if (transliterated !== startValue) {
      // check that the input hasn't changed in the meantime:
      if (inputElement.value == startValue) {
        inputElement.value = transliterated;
      }
    }
  }

  async submitAnswer() {
    let input = document.getElementById("input");
    this.updateState({
      answers: this.state.answers.concat([input.value]),
    });
    input.value = "";

    let expectedAnswers = this.expectedAnswers();
    if (this.state.answers.length < expectedAnswers.length) {
      return;
    }

    return;
  }
}

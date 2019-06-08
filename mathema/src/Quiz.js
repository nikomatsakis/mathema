/*eslint no-unused-vars: ["error", { "varsIgnorePattern": "[iI]gnored" }]*/

// CURRENT STATUS:
//
// If we get back to the quiz main state, check if `missingAnswers` is non-empty
// and ask user yes/no/almost.

import React, { Component } from 'react';
import log from 'picolog';
import { HOST } from "./Constants";
import Question from "./Question";
import Card from "./Card";
import Answers from "./Answers";
import MissingAnswers from "./MissingAnswers";
import QuizComplete from "./QuizComplete";
import {post} from "./util";

// Properties:
//
// - language: to quiz
// - duration: in seconds
export default class QuizComponent extends Component {
  state = {
    // List of all questions we will ask during the quiz
    questions: null,

    // The date when we started the quiz, recorded once words are loaded
    startTime: null,

    // Current index into the list
    index: 0,

    // Current card we are showing to the user
    card: null,

    // Answers the user has given thus far; a list of [answer, index] tuples
    // where:
    //
    // - answer is the string
    // - index is the index of the answer from the list of expected answers
    //   or undefined
    answers: [],

    // If this is non-empty, then the user has given all their answers,
    // but there were some mistakes. We need to ask them how they feel
    // about their work...did they know it...yes/no/almost?
    missingAnswers: [],

    // List of pending transliteration promises. We append each
    // transliteration request to this list as the user types. When
    // they hit enter, we wait for them all to complete (most will
    // have already done so, but so what). We set the list to
    // null once we have started to submit forms.
    //
    // This doesn't feel like it belongs in the react state but I'm
    // not sure where else to put it.
    pendingTransliterations: null,
  };

  componentDidMount() {
    this.loadWords();
    this.keyupListener = (event) => this.keyupEvent(event);
    document.addEventListener('keyup', this.keyupListener);
  }

  componentDidUpdate() {
    // Whenever we are showing a "[...]" textbox on the screen,
    // we want it to be focused.
    let inputElement = document.getElementById("input");
    if (inputElement != null)
      inputElement.focus();
  }

  compountWillUnmount() {
    document.removeEventListener('keyup', this.keyupListener);
  }

  updateState(obj) {
    log.log(`quiz: update state=${JSON.stringify(obj)}`);
    this.setState(Object.assign(this.state, obj));
  }

  async loadWords() {
    let language = this.props.language;

    let questions = await fetch(`${HOST}/quiz_cards/${language}`)
        .then(r => r.json())
        .then(json => {
          return json.map(q => Question.fromJson(q));
        });

    this.updateState({ startTime: Date.now() });
    this.updateState({ questions, index: 0 });
    this.resetStateBetweenQuestions();
    this.loadCard();
  }

  resetStateBetweenQuestions() {
    this.updateState({
      card: null,
      answers: [],
      missingAnswers: [],
      pendingTransliterations: [],
    });
  }

  async loadCard() {
    let uuid = this.state.questions[this.state.index].uuid;
    log.log("loadCard(): uuid = " + uuid);
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

    if (questions === null) {
      return (
          <div className="container">
          <div className="col-xs-12">
          <h1>Loading questions...</h1>
          </div>
          </div>
      );
    }

    // Asked all the cards.
    if (index >= questions.length) {
      return (<QuizComplete/>);
    }

    if (card === null) {
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
      if (this.state.pendingTransliterations != null) {
        this.updateState({
          pendingTransliterations: this.state.pendingTransliterations.concat([
            this.translateInput(target, startValue)
          ])
        });
      } else {
        // we've already started to submit, just wait
      }
    };

    let submitAnswer = (event) => {
      event.preventDefault();
      let input = document.getElementById("input");

      if (this.state.pendingTransliterations != null) {
        Promise.all(this.state.pendingTransliterations).then(this.submitAnswer(input, input.value));
        this.updateState({
          pendingTransliterations: null,
        });
      } else {
        // we've already started to submit, just wait
      }
    };

    let answerYes = (event) => {
      event.preventDefault();
      this.submitResult("yes")
    };

    let answerNo = (event) => {
      event.preventDefault();
      this.submitResult("no")
    };

    let answerAlmost = (event) => {
      event.preventDefault();
      this.submitResult("almost")
    };

    return (
        <div className="container">
        <div className="col-xs-12">
        <h1>Translate to {toLanguage}</h1>
        <ul>
        {card.meaningsIn(fromLanguage).map((meaning, index) => (
            <li key={`meaning-${index}`}> {meaning} </li>
        ))}
         </ul>

        <Answers answers={this.state.answers}/>

      {this.state.missingAnswers.length > 0 ? (
          <div onKeyPress={(e) => log.log(e.key)}>
          <MissingAnswers missingAnswers={this.state.missingAnswers}/>
          <h3>Did you know it?</h3>
          <form>
          <ul>
          <li> <button onClick={answerYes}>Yes</button> </li>
          <li> <button onClick={answerNo}>No</button> </li>
          <li> <button onClick={answerAlmost}>Almost...</button> </li>
          </ul>
          </form>
          </div>
      ) : (
        <div>
        <h3>Your answer</h3>

       <form onSubmit={submitAnswer}>
          <input type="text" id="input" ref={this.inputRef} name="input" size="50" onInput={translateInput}></input>
          </form>
          </div>
      )}

         </div>
         </div>
        );
  }

  async translateInput(inputElement, startValue) {
    // If no data, nothing to transliterate.
    if (startValue.length === 0)
      return;

    // If the input has changed since we were scheduled, just
    // abort. There will be another event scheduled.
    if (inputElement.value !== startValue)
      return;

    let question = this.state.questions[this.state.index];
    let toLanguage = question.questionKind.toLanguage();
    let uri = `${HOST}/transliterate/${encodeURIComponent(toLanguage)}/${encodeURIComponent(startValue)}`;
    let transliterated = await fetch(uri).then(r => r.json());
    if (transliterated !== startValue) {
      // Check that the input hasn't changed in the meantime (i.e.,
      // because the user typed more). If it did, we'll have kicked
      // off a separate request for that.
      if (inputElement.value === startValue) {
        inputElement.value = transliterated;
      }
    }
  }

  // Given the html doc element for the `<input>` and the answer text
  // that was given, submit the answer to the server.
  async submitAnswer(input, answer) {
    log.log("submitAnswer");

    // Whatever happens, clear out the text box the user wrote.
    input.value = "";

    // Whenever the user presses enter with an empty box, that's the end of this question
    if (answer === "") {
      return this.determineGrade();
    }

    let answerIndex = await this.answerIndex(answer);

    // Check if we already *had* this answer, or something equivalent
    // to it.  If so, just clear it and reset back.
    for (let [oldAnswer, oldAnswerIndex] of this.state.answers) {
      if (answer === oldAnswer || (oldAnswerIndex !== undefined && oldAnswerIndex === answerIndex)) {
        return this.expectMoreAnswers();
      }
    }

    // Otherwise, figure out if it is correct and add it to our list of answers.
    this.updateState({
      answers: this.state.answers.concat([[answer, answerIndex]]),
    });

    // Now, if we still expect more answers, wait for them.
    let expectedAnswers = this.expectedAnswers();
    if (this.state.answers.length < expectedAnswers.length) {
      return this.expectMoreAnswers();
    }

    // Otherwise...
    await this.determineGrade();
  }

  expectMoreAnswers() {
    log.log("expectMoreAnswers");

    this.updateState({
      pendingTransliterations: [],
    });
    return;
  }

  async determineGrade() {
    log.log(`determineGrade()`);

    let expectedAnswers = this.expectedAnswers();
    let missingAnswers = expectedAnswers.slice(0); // make a local clone
    for (let [answerIgnored, answerIndex] in this.state.answers) {
      if (answerIndex !== undefined) {
        missingAnswers[answerIndex] = null;
      }
    }
    missingAnswers = missingAnswers.filter(a => a != null);

    if (missingAnswers.length === 0) {
      await this.submitResult("yes");
    } else {
      this.updateState({missingAnswers});
    }

    return;
  }

  async checkAnswer(expected, user) {
    let uri = `${HOST}/check_answer/` +
        `${encodeURIComponent(expected)}/` +
        `${encodeURIComponent(user)}`;
    await fetch(uri).then(r => r.json())
  }

  // Returns the index of answer in the list of expected answers, or else undefined.
  async answerIndex(answer) {
    let expectedAnswers = this.expectedAnswers();
    for (let i = 0; i < expectedAnswers.length; i++) {
      if (await this.checkAnswer(expectedAnswers[i], answer)) {
        return i;
      }
    }
    return undefined;
  }

  async keyupEvent(event) {
    if (this.state.missingAnswers.length > 0) {
      // waiting for a yes, no, almost...
      log.log(`received keypress: ${event.code}`);
      if (event.code === "KeyY") {
        this.submitResult("yes");
      } else if (event.code === "KeyN") {
        this.submitResult("no");
      } else if (event.code === "KeyA") {
        this.submitResult("almost");
      }
    }
  }

  async submitResult(result) {
    log.log(`submitResult(${result})`);

    let question = this.state.questions[this.state.index];
    let uuid = question.uuid;
    let fromLanguage = question.questionKind.fromLanguage();
    let toLanguage = question.questionKind.toLanguage();

    let uri = `${HOST}/mark_answer/` +
        encodeURIComponent(uuid) + "/translate/" +
        encodeURIComponent(fromLanguage) + "/" +
        encodeURIComponent(toLanguage) + "/" +
        result;
    await post(uri, "");

    this.resetStateBetweenQuestions();
    let timeThusFar = (Date.now() - this.state.startTime) / 1000; // in seconds
    if (timeThusFar > this.props.duration) {
      // Ran out of time
      log.info(`quiz duration exceeded: timeThusFar=${timeThusFar}`);
      this.updateState({
        index: this.state.questions.length,
      });
    } else {
      this.updateState({
        index: this.state.index + 1,
      });
      this.loadCard();
    }
  }
}



// States:
//
//
export default class QuizState {
  constructor() {
    this.questions = null;
    this.index = 0;
    this.card = null;
  }

  withFields(object) {
    return Object.assign(this, object);
  }

  withLoadedQuestions(questions) {
    return this.withFields({
      questions: questions,
      index: 0,
      card: null,
    });
  }

  withLoadedCard(card) {
    return this.withFields({
      card: card,
    });
  }
}

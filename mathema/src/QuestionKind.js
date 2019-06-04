// Represents the rust type `QuestionKind`. You create it with a JSON
// object. Presently the only real sort of question is a `Translate`
// question, though in theory we might add more in the future.
export default class QuestionKind {
  constructor() {
  }

  static fromJson(json) {
    return Object.assign(new QuestionKind(), json);
  }

  isTranslate() {
    return "Translate" in this.json;
  }

  fromLanguage() {
    return this.Translate.from;
  }

  toLanguage() {
    return this.Translate.to;
  }

  expectedAnswers(card) {
    return card.meaningsIn(this.toLanguage());
  }
}

import QuestionKind from "./QuestionKind";

// Represents a question in a quiz. A question is the pair of a UUID
// (for a card) and a question-kind.
export default class Question {
  constructor(uuid, questionKind) {
    this.uuid = uuid
    this.questionKind = questionKind;
  }

  static fromJson(json) {
    return new Question(json[0], QuestionKind.fromJson(json[1]));
  }
}

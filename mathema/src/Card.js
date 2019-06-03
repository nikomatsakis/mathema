import { HOST } from "./Constants";

export default class Card {
  constructor(uuid) {
    this.uuid = uuid;
    this.meanings = {};
    this.conjugations = {};
    this.partOfSpeech = null;
  }

  static async fetch(cardUuid) {
    console.log("hi");
    console.log(`Card.fetch(cardUuid=${cardUuid})`);
    let cardData = await fetch(`${HOST}/card/${cardUuid}`).then(r => r.json());
    let card = new Card(cardUuid);
    for (let line of cardData.lines) {
      if (line.kind === "PartOfSpeech") {
        card.partOfSpeech = line.text;
      } else if (line.kind.Meaning !== undefined) {
        card.meanings[line.kind.Meaning] = line.text;
      } else if (line.kind === "Comment") {
      } else if (line.kind === "Aoristos") {
        card.conjugations[line.kind] = line.text;
      } else {
        throw new Error("Unrecognized line on card from " + card.start_line + ":" + JSON.stringify(line));
      }
    }
    return card;
  }
}


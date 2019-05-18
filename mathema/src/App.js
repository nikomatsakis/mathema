import React, { Component } from 'react';

class Card {
  constructor(uuid) {
    this.uuid = uuid;
    this.meanings = {};
    this.conjugations = {};
    this.partOfSpeech = null;
  }
}

class App extends Component {
  state = {
    cards: []
  }

  componentDidMount() {
    fetchData(this);
  }

  render() {
    return (
      <div className="container">
        <div className="col-xs-12">
          <h1>My Todos</h1>
          {this.state.cards.map((card) => (
            <div className="card">
              <div className="card-body">
                <h5 className="card-title">{card.uuid}</h5>
                <h6 className="card-subtitle mb-2 text-muted">
                  {card.meanings.English}, {card.meanings.Greek}
                </h6>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }
}

async function fetchData(app) {
  const host = "/api";
  let cardUuids = await fetch(`${host}/cards`).then(r => r.json());
  let data = await Promise.all(cardUuids.map(async function (cardUuid) {
    let cardData = await fetch(`${host}/card/${cardUuid}`).then(r => r.json());
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
  }));
  app.setState({ cards: data });
}

export default App;

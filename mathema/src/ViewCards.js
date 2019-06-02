import React, { Component } from 'react';
import Card from "./Card";
import { Host } from "./Constants";

export default class ViewCards extends Component {
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
  let cardUuids = await fetch(`${HOST}/cards`).then(r => r.json());
  let data = await Promise.all(cardUuids.map(async function (cardUuid) {
    return await Card.fetch(cardUuid);
  }));
  app.setState({ cards: data });
}

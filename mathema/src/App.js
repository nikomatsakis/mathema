import React, { Component } from 'react';

class App extends Component {
  state = {
    todos: []
  }

  componentDidMount() {
    fetchData(this);
  }

  render() {
    return (
        <div className="container">
        <div className="col-xs-12">
        <h1>My Todos</h1>
        {this.state.todos.map((todo) => (
            <div className="card">
            <div className="card-body">
            <h5 className="card-title">{todo.title}</h5>
            <h6 className="card-subtitle mb-2 text-muted">
            { todo.completed &&
              <span>
              Completed
              </span>
            }
          { !todo.completed &&
            <span>
            Pending
            </span>
          }
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
  let result = await fetch('http://jsonplaceholder.typicode.com/todos');
  let data = await result.json();
  app.setState({ todos: data })
  console.log(app.state.todos)
}

export default App;

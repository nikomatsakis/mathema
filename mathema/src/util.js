import log from "picolog";

// Posts the given value to the given URL and returns a promise with its response.
export function post(url, value) {
  log.log(`posting ${value} to ${url}`);

  return fetch(url, {
    method: 'POST',
    body: JSON.stringify(value),
    headers:{
      'Content-Type': 'application/json'
    }
  });
}

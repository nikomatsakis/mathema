import log from "picolog";

// Posts the given value to the given uri in a "fire and forget" fashion.
export function post(url, value) {
  log.log(`posting ${value} to ${url}`);

  var xhr = new XMLHttpRequest();
  xhr.open("POST", url, true);
  xhr.setRequestHeader('Content-Type', 'application/json');
  xhr.send(JSON.stringify(value));
}

import log from "picolog";

// from http://ccoenraets.github.io/es6-tutorial-data/promisify/
function xhrPromise(obj) {
  return new Promise((resolve, reject) => {
    let xhr = new XMLHttpRequest();
    xhr.open(obj.method || "GET", obj.url);
    if (obj.headers) {
      Object.keys(obj.headers).forEach(key => {
        xhr.setRequestHeader(key, obj.headers[key]);
      });
    }
    xhr.onload = () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve(xhr.response);
      } else {
        reject(xhr.statusText);
      }
    };
    xhr.onerror = () => reject(xhr.statusText);
    xhr.send(obj.body);
  });
}

// Posts the given value to the given URL and returns a promise with its response.
export function post(url, value) {
  log.log(`posting ${value} to ${url}`);

  return xhrPromise({
    method: "POST",
    url: url,
    body: JSON.stringify(value),
  });
}

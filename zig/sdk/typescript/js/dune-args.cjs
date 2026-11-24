"use strict";

module.exports = function duneArgs(options, id, field, post) {
  if (!id) {
    if (!Object.hasOwn(options, "request")) return options;
    if (Object.hasOwn(options, "body")) throw new TypeError("supply the request once");
    return {body:options.request};
  }
  if (!Object.hasOwn(options, id)) return options;
  if (Object.hasOwn(options, field) || Object.hasOwn(options.request ?? {}, field)) {
    throw new TypeError("supply the identifier once");
  }
  const request = options.request ?? {};
  return post ? {[field]:options[id],body:request} : {[field]:options[id],...request};
};

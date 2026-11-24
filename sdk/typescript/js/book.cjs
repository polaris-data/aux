"use strict";
module.exports = function book(stream) {
  return {
    [Symbol.asyncIterator]() { return this; },
    async next() {
      for (;;) {
        const result=await stream.next();
        if (result.done) return result;
        if (result.value.channel === "l2Book") return {done:false,value:result.value.data};
      }
    },
    close() { stream.close(); },
    async return() { stream.close(); return {done:true,value:undefined}; },
  };
};

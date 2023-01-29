const fs = require("fs");
const ALPHABET = "[abcdefghijklmnopqrstuvwxyz]";

const words = fs.readFileSync("./fivechars.txt").toString().trim().split("\n");

class Cheatle {
  constructor() {
    this.reset();
  }

  reset() {
    this.remaining = [...Array(5).keys()].map(x => ALPHABET);
    this.required = {};
    this.indices = [...Array(words.length).keys()];
  }

  removeLetter(pos, letter) {
    this.remaining[pos] = this.remaining[pos].replace(letter, "");
  }

  notInWord(letter) {
    [...Array(5).keys()].forEach((pos) => this.removeLetter(pos, letter));
  }

  placedInWord(pos, letter) {
    this.remaining[pos] = letter;
  }

  misplacedInWord(pos, letter) {
    this.removeLetter(pos, letter);
    if (!(letter in this.required)) {
      this.required[letter] = 1;
    }
  }

  setMinOccurences(letter, count) {
    this.required[letter] = count;
  }

  hasRequired(word) {
    for (let letter in this.required) {
      const count = this.required[letter];
      // Count instances of letter in word
      const countInWord = word.split(letter).length - 1;
      if (count > countInWord) {
        return false;
      }
    }
    return true;
  }

  filter() {
    const expr = this.remaining.join("");
    const prog = new RegExp(expr);
    console.log("indices.length = " + this.indices.length);
    this.indices = this.indices.filter((index) => {
      const word = words[index];
      return (this.hasRequired(word) && prog.test(word));
    });
  }

  getWords() {
    return this.indices.map((index) => words[index]);
  }
}

module.exports = { Cheatle };

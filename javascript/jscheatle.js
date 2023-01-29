// import * as readline from 'node:readline/promises';
const readline = require("readline/promises");
// import * as readline from 'node:readline/promises';
const cheatle = require("./cheatle");

const printHelp = () => {
  console.log("reset : start a new cheating session");
  console.log("- <LETTERS> : letters not in solution");
  console.log("-N <LETTER> : letter in solution but not in Nth position");
  console.log("+N <LETTER> : letter is in Nth position");
  console.log(">=N <LETTER>: letter occurs at least N time in word");
  console.log("list : list remaining solutions");
  console.log("help : this messsage");
}

async function mainLoop() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.on("close", () => {
    console.log("\nBye!");
  });

  let chtl = new cheatle.Cheatle();

  while (true) {
    const line = await rl.question("cheatle> ");
    const tokens = line.split(" ");
    if (tokens.length == 0) {
      continue;
    }
    const command = tokens[0];
    if (command == 'reset') {
      chtl.reset();
    } else if (command == 'help') {
      printHelp();
    } else if (command == '-') {
      if (tokens.length != 2) {
        printHelp();
        continue;
      }
      for (let c of tokens[1]) {
        chtl.notInWord(c)
      }
    } else if (['-1', '-2', '-3', '-4', '-5'].includes(command)) {
      if (tokens.length != 2) {
        printHelp();
        continue;
      }
      const pos = parseInt(command.slice(1)) - 1;
      chtl.misplacedInWord(pos, tokens[1]);
    } else if (['+1', '+2', '+3', '+4', '+5'].includes(command)) {
      if (tokens.length != 2) {
        printHelp();
        continue;
      }
      const pos = parseInt(command.slice(1)) - 1;
      chtl.placedInWord(pos, tokens[1]);
    } else if (['>=1', '>=2', '>=3', '>=4', '>=5'].includes(command)) {
      if (tokens.length != 2) {
        printHelp();
        continue;
      }
      const count = parseInt(command.slice(2));
      chtl.setMinOccurences(tokens[1], count);
    } else if (command == 'list') {
      chtl.filter();
      const words = chtl.getWords();
      console.log(words.join("\n"));
      console.log("count: " + words.length);
    } else {
      console.log("Ivalid command");
      printHelp();
    }
  }
}

mainLoop();

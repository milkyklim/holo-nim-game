const path = require('path')
const tape = require('tape')

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/nim.dna.json")
const dna = Diorama.dna(dnaPath, 'nim')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})


// <<DEVCAMP>> Your tests here
// uncomment one of these
// require('./tictactoe')(diorama.registerScenario)
// require('./checkers')(diorama.registerScenario)
require('./nim')(diorama.registerScenario)

// test the matchmaking 
require('./matchmaking')(diorama.registerScenario)

diorama.run()
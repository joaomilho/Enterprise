module.exports = {
  fn: "read",
  code: `
  const ___rl = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });

  const read = (question) => new Promise(resolve => ___rl.question(question, answer => {
    resolve(answer)
    ___rl.close()
  }))
  `,
  type: "String"
};

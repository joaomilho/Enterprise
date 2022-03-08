/*©
  This code is property of Enterprise™.
©*/

const { highlight } = require('cli-highlight')

const parseAST = require('./ast')
const fs = require('fs')
const { gray, red, green, yellow, bold } = require('colors/safe')

const compile = (ast) => {
  return ast.map(compileNode).filter(Boolean).join('\n')
}

const delay = (fnCall) => {
  return global.turbo
    ? fnCall
    : `await new Promise(r => {
    setTimeout(async () => { r(await ${fnCall}) }, 1000)
  })`
}

const compileNode = (node) => {
  switch (node.type) {
    case 'comment':
      return null
    case 'import':
      return require(`./disruptiveLibs/${node.lib}`).code
    case 'finalDisruptiveClass':
      return `
      class ${node.name} { ${compile(node.body)} }
      new ${node.name}().main()`
    case 'mainMethod':
      return `async main () {
        ${compile(node.body)}
      }`
    case 'var':
      return `var ${node.name} = ${compileNode(node.value)}`
    case 'while':
      return `while (${compileNode(node.test)}) {
        ${compile(node.body)}
      }`
    case 'if':
      return `if (${compileNode(node.test)}) {
        ${compile(node.then)}
      } else {
        ${compile(node.else)}
      }`
    case 'binary':
      return `${compileNode(node.left)} ${node.operator} ${compileNode(
        node.right
      )}`
    case 'call':
      return delay(`${node.callee}(${node.args.map(compileNode).join(', ')})`)
    case 'mutate':
      return `${node.var}${node.mutation}`
    case 'varName':
    case 'numVarName':
      return node.value
    default:
      return JSON.stringify(node)
  }
}

const CompileError = require('./CompileError')
const dls = require('./dls')

module.exports = (file, turbo) => {
  global.turbo = turbo

  let ast
  try {
    ast = parseAST(file)
    return compile(ast)
  } catch (e) {
    if (e instanceof CompileError) {
      displayError(e, file)
    } else {
      throw e
    }
  }
}

const pad = (len) => (idx) => {
  // const padCount = len - (idx + 1).length.toString().length + 1;

  return gray(`${idx + 1}`.padStart(len + 1) + ' |')
}

const code = (file, location) => {
  const sourceLines = fs.readFileSync(file, 'utf8').split('\n')
  const len = sourceLines.length.toString().length
  const padLen = pad(len)
  const r = sourceLines.map((line, i) => `${padLen(i)} ${line}`)

  r.splice(
    location.start.line,
    0,
    `${''.padEnd(len + 4)}${gray(
      ''.padEnd(location.start.column - 1, '-')
    )}${red(''.padEnd(location.end.column - location.start.column, '^'))}`
  )
  return r.join('\n')
}

const displayError = (e, file) => {
  switch (e.type) {
    case 'invalid_lib':
      console.log('\n')
      console.log(red(`Invalid library ${bold(e.value)}`), '\n')
      console.log(`Pick one of: \n  - `, Object.keys(dls).join('\n  - '))
      console.log(gray('\n~~~~~~~~~~~~~~~~~~~~~'))
      console.log(code(file, e.location))
      console.log(gray('~~~~~~~~~~~~~~~~~~~~~\n'))
    case 'invalid_call':
      console.log('\n')
      console.log(red(`Invalid call ${bold(e.value)}`), '\n')
      console.log(gray('\n~~~~~~~~~~~~~~~~~~~~~'))
      console.log(code(file, e.location))
      console.log(gray('~~~~~~~~~~~~~~~~~~~~~\n'))
  }
}

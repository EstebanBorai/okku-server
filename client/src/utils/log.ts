export default {
  blue: (text: string) => console.log('\x1b[34m%s\x1b[0m', text),
  cyan: (text: string) => console.log('\x1b[36m%s\x1b[0m', text),
  green: (text: string) => console.log('\x1b[32m%s\x1b[0m', text),
  magenta: (text: string) => console.log('\x1b[35m%s\x1b[0m', text),
  red: (text: string) => console.log('\x1b[31m%s\x1b[0m', text),
  yellow: (text: string) => console.log('\x1b[36m%s\x1b[0m', text),
}

// program to display fibonacci sequence using recursion
function fibonacci(num) {
  if (num < 2) {
    return num
  } else {
    return fibonacci(num - 1) + fibonacci(num - 2)
  }
}

performance.mark('start')
const sum = fibonacci(50)
performance.mark('end')

performance.measure('sum', 'start', 'end')

const [entry] = performance.getEntriesByName('sum')

console.log(entry.duration / 1000)

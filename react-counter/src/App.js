import { useState } from 'react'

function Counter() {
  const [count, setCount] = useState(0)

  const increment = () => setCount(count + 1)

  return (
    <>
      <div>{count}</div>
      <button onclick={increment}>{'Increment'}</button>
    </>
  )
}

export default Counter

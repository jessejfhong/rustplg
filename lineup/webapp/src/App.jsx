import { useState } from 'react';

export default function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <h1 className="text-3xl font-bold underline">
        Hello world!
      </h1>
      <button onClick={() => setCount(count => count + 1)}
        className="bg-blue-500 hover:bg-blue-700 text-white font-blod py-2 px-4 rounded">
        count is {count}
      </button>
    </>
  );
}

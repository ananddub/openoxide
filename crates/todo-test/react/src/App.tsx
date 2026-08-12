import {FormEvent, useEffect, useState} from 'react';
import {type Todo, useTodos} from 'virtual:openoxide-live';

async function mutate(path: string, method = 'POST', body?: unknown) {
  const response = await fetch(path, {method, headers: body ? {'content-type': 'application/json'} : undefined, body: body ? JSON.stringify(body) : undefined});
  if (!response.ok) throw new Error(`Request failed: ${response.status}`);
}

export function App() {
  const [initial, setInitial] = useState<Todo[] | null>(null);
  const [title, setTitle] = useState('');

  useEffect(() => { fetch('/api/todos').then((r) => r.json()).then(setInitial); }, []);
  if (!initial) return <main className="shell"><p>Loading todos…</p></main>;
  return <TodoApp initial={initial} title={title} setTitle={setTitle} />;
}

function TodoApp({initial, title, setTitle}: {initial: Todo[]; title: string; setTitle: (value: string) => void}) {
  const live = useTodos();
  const todos = live.data ?? initial;
  const connected = live.connected;
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (!title.trim()) return;
    await mutate('/api/todos', 'POST', {title});
    setTitle('');
  };

  return <main className="shell">
    <header><div><h1>Realtime Todos</h1><p>Open two tabs and edit either one.</p></div><span className={connected ? 'status online' : 'status'}>{connected ? 'Live' : 'Offline'}</span></header>
    <form onSubmit={submit}><input value={title} onChange={(e) => setTitle(e.target.value)} placeholder="What needs doing?"/><button>Add</button></form>
    <section>{todos.length === 0 ? <p className="empty">No todos yet.</p> : todos.map((todo) =>
      <article key={todo.id}>
        <input type="checkbox" checked={todo.done} onChange={() => mutate(`/api/todos/${todo.id}/toggle`)}/>
        <span className={todo.done ? 'done' : ''}>{todo.title}</span>
        <button className="delete" onClick={() => mutate(`/api/todos/${todo.id}`, 'DELETE')}>Delete</button>
      </article>
    )}</section>
    <footer>{todos.filter((todo) => !todo.done).length} remaining · {todos.filter((todo) => todo.done).length} completed</footer>
  </main>;
}

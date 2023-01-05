import React, { useState, useEffect } from 'react';
import axios from 'axios';

function TodoTracker() {
    const [todos, setTodos] = useState([]);
    const [newTodo, setNewTodo] = useState('');

    useEffect(() => {
        // Fetch the todos from the Rust service when the component mounts
        axios.get('http://localhost:8000/todos').then(response => {
            setTodos(response.data);
        });
    }, []);

    function handleSubmit(event) {
        event.preventDefault();
        // Add the new todo to the Rust service
        axios.post('http://localhost:8000/todos', {id:0, title: newTodo, completed: false }).then(response => {
            setTodos([...todos, response.data]);
            setNewTodo('');
        });
    }

    function handleDelete(id) {
        // Remove the todo from the Rust service
        axios.delete(`http://localhost:8000/todos/${id}`).then(() => {
            setTodos(todos.filter(todo => todo.id !== id));
        });
    }

    function handleToggle(id) {
        // Toggle the "completed" status of the todo in the Rust service
        const todo = todos.find(todo => todo.id === id);
        axios.patch(`http://localhost:8000/todos/${id}`, {id: todo.id, title: todo.title, completed: !todo.completed }).then(response => {
            setTodos(todos.map(todo => (todo.id === id ? response.data : todo)));
        });
    }

    return (
        <div>
            <h1>Todo Tracker</h1>
            <form onSubmit={handleSubmit}>
                <input
                    type="text"
                    value={newTodo}
                    onChange={event => setNewTodo(event.target.value)}
                />
                <button type="submit">Add Todo</button>
            </form>
            <ul>
                {todos.map(todo => (
                    <li key={todo.id}>
                        <input
                            type="checkbox"
                            checked={todo.completed}
                            onChange={() => handleToggle(todo.id)}
                        />
                        {todo.title}
                        <button onClick={() => handleDelete(todo.id)}>Delete</button>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default TodoTracker;

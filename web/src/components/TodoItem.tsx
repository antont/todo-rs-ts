import { useState, useRef, useEffect } from 'react'
import type { Todo } from '../types/generated'

interface Props {
  todo: Todo
  onToggle: (completed: boolean) => void
  onUpdate: (title: string) => void
  onDelete: () => void
}

export function TodoItem({ todo, onToggle, onUpdate, onDelete }: Props) {
  const [editing, setEditing] = useState(false)
  const [editText, setEditText] = useState(todo.title)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus()
    }
  }, [editing])

  const handleDoubleClick = () => {
    setEditing(true)
    setEditText(todo.title)
  }

  const handleSubmit = () => {
    const title = editText.trim()
    if (title) {
      onUpdate(title)
      setEditing(false)
    } else {
      onDelete()
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSubmit()
    } else if (e.key === 'Escape') {
      setEditText(todo.title)
      setEditing(false)
    }
  }

  const className = [
    todo.completed ? 'completed' : '',
    editing ? 'editing' : '',
  ]
    .filter(Boolean)
    .join(' ')

  return (
    <li className={className}>
      <div className="view">
        <input
          className="toggle"
          type="checkbox"
          checked={todo.completed}
          onChange={() => onToggle(!todo.completed)}
        />
        <label onDoubleClick={handleDoubleClick}>{todo.title}</label>
        <button className="destroy" onClick={onDelete} />
      </div>
      {editing && (
        <input
          ref={inputRef}
          className="edit"
          value={editText}
          onChange={(e) => setEditText(e.target.value)}
          onBlur={handleSubmit}
          onKeyDown={handleKeyDown}
        />
      )}
    </li>
  )
}

interface Props {
  activeCount: number
  completedCount: number
  filter: string
  onFilterChange: (filter: string) => void
  onClearCompleted: () => void
}

const FILTERS = ['all', 'active', 'completed'] as const

export function TodoFooter({
  activeCount,
  completedCount,
  filter,
  onFilterChange,
  onClearCompleted,
}: Props) {
  return (
    <footer className="footer">
      <span className="todo-count">
        <strong>{activeCount}</strong> {activeCount === 1 ? 'item' : 'items'} left
      </span>
      <ul className="filters">
        {FILTERS.map((f) => (
          <li key={f}>
            <a
              href={`#/${f}`}
              className={filter === f ? 'selected' : ''}
              onClick={() => onFilterChange(f)}
            >
              {f[0].toUpperCase() + f.slice(1)}
            </a>
          </li>
        ))}
      </ul>
      {completedCount > 0 && (
        <button className="clear-completed" onClick={onClearCompleted}>
          Clear completed
        </button>
      )}
    </footer>
  )
}

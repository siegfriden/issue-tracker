import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <div>
      <h1>Issue Tracker</h1>
    </div>
  </StrictMode>,
)

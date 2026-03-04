import { Outlet } from 'react-router-dom'
import './App.css'

function App() {

  return (
    <>
    <main>
        <div>
          <Outlet />
        </div>
      </main>
    </>
  )
}

export default App

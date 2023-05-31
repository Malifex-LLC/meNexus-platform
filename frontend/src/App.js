import './App.css';
import { BrowserRouter, Router, Routes, Route} from 'react-router-dom';
import Home from './components/Pages/Home';
import Profile from "./components/Pages/Profile";
import Register from "./components/Pages/Register";
import Login from "./components/Pages/Login";



function App() {

  return (
    <div className="App">
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Register/>}/>
                <Route path="/login" element={<Login/>}/>
                <Route path="/home" element={<Home/>}/>
                <Route path='/profile' element={<Profile/>}/>
            </Routes>
        </BrowserRouter>
    </div>
  );
  
}

export default App;

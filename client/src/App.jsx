import './App.css';
import { BrowserRouter, Router, Routes, Route} from 'react-router-dom';
import HomePage from './components/Pages/HomePage.jsx';
import RegisterPage from "./components/Pages/RegisterPage.jsx";
import LoginPage from "./components/Pages/LoginPage.jsx";
import UserProfilePage from "./components/Pages/UserProfilePage.jsx";



function App() {

    return (
        <div className="App">
            <BrowserRouter>
                <Routes>
                    <Route path="/register" element={<RegisterPage/>}/>
                    <Route path="/login" element={<LoginPage/>}/>
                    <Route path="/home" element={<HomePage/>}/>
                    <Route path='/profile/:handle' element={<UserProfilePage/>}/>
                </Routes>
            </BrowserRouter>
        </div>
    );

}

export default App;

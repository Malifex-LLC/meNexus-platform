import './App.css';
import { BrowserRouter, Router, Routes, Route} from 'react-router-dom';
import HomePage from './pages/Home/HomePage.jsx';
import RegisterPage from "./pages/Register/RegisterPage.jsx";
import LoginPage from "./pages/Login/LoginPage.jsx";
import UserProfilePage from "./pages/UserProfile/UserProfilePage.jsx";



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

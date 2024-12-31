import './App.css';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import HomePage from './pages/Home/HomePage.jsx';
import RegisterPage from "./pages/Register/RegisterPage.jsx";
import LoginPage from "./pages/Login/LoginPage.jsx";
import UserProfilePage from "./pages/UserProfile/UserProfilePage.jsx";
import SettingsPage from "./pages/Settings/SettingsPage.jsx";
import SearchPage from "./pages/Search/SearchPage.jsx";
import MessagesPage from "./pages/Messages/MessagesPage.jsx";

function App() {
    return (
        <div className="App">
            <BrowserRouter>
                <Routes>
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/login" element={<LoginPage />} />

                    {/* /home redirects to /home/:handle based on session user */}
                    <Route path="/home" element={<HomePage />} />
                    <Route path="/home/:handle" element={<HomePage />} />

                    {/* /profile redirects to /profile/:handle based on session user */}
                    <Route path="/profile" element={<UserProfilePage />} />
                    <Route path="/profile/:handle" element={<UserProfilePage />} />

                    <Route path="/messages" element={<MessagesPage />} />

                    <Route path='/settings' element={<SettingsPage />} />

                    <Route path='/search' element={<SearchPage />} />

                    {/* Default fallback */}
                    <Route path="*" element={<Navigate to="/login" replace />} />
                </Routes>
            </BrowserRouter>
        </div>
    );
}

export default App;
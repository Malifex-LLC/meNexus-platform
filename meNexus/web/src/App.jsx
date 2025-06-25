import './App.css';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import HomePage from './pages/Home/HomePage.jsx';
import SynapsePage from './pages/Synapse/SynapsePage.jsx';
import SynapseExplorePage from './pages/Synapse/SynapseExplorePage.jsx';
import RegisterPage from "./pages/Register/RegisterPage.jsx";
import LoginPage from "./pages/Login/LoginPage.jsx";
import UserProfilePage from "./pages/UserProfile/UserProfilePage.jsx";
import SettingsPage from "./pages/Settings/SettingsPage.jsx";
import SearchPage from "./pages/Search/SearchPage.jsx";
import MessagesPage from "./pages/Messages/MessagesPage.jsx";
import PostPage from "./pages/Post/PostPage.jsx";
import DashboardPage from "./pages/Dashboard/DashboardPage.jsx";
import ExplorePage from "./pages/Explore/ExplorePage.jsx";

function App() {
    return (
        <div className="App">
            <BrowserRouter>
                <Routes>
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/login" element={<LoginPage />} />

                    <Route path="/dashboard" element={<DashboardPage />} />

                    {/* /home redirects to /home/:handle based on session user */}
                    <Route path="/home" element={<HomePage />} />
                    <Route path="/home/:handle" element={<HomePage />} />

                    <Route path="/explore" element={<ExplorePage />} />

                    <Route path="/synapse/:synapsePublicKey" element={<SynapsePage />} />

                    {/* /profile redirects to /profile/:handle based on session user */}
                    <Route path="/profile" element={<UserProfilePage />} />
                    <Route path="/profile/:handle" element={<UserProfilePage />} />

                    <Route path="/post/:postId" element={<PostPage />} />

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
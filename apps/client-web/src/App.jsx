// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import './App.css';
import {useEffect, useState} from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import HomePage from './pages/Home/HomePage.jsx';
import SynapsePage from './pages/Synapse/SynapsePage.jsx';
import RegisterPage from "./pages/Register/RegisterPage.jsx";
import LoginPage from "./pages/Login/LoginPage.jsx";
import UserProfilePage from "./pages/UserProfile/UserProfilePage.jsx";
import SettingsPage from "./pages/Settings/SettingsPage.jsx";
import SearchPage from "./pages/Search/SearchPage.jsx";
import MessagesPage from "./pages/Messages/MessagesPage.jsx";
import PostPage from "./pages/Post/PostPage.jsx";
import DashboardPage from "./pages/Dashboard/DashboardPage.jsx";
import ExplorePage from "./pages/Explore/ExplorePage.jsx";
import useThemeLoader from './api/hooks/useThemeLoader.js';
import RootLayout from "./layouts/RootLayout/RootLayout.jsx";

function App() {
    const [theme, setTheme] = useState(() => localStorage.getItem('theme') || 'SalmonRoasted');

    useEffect(() => {
        localStorage.setItem('theme', theme);
    }, [theme]);
    useThemeLoader(theme);

    return (
        <div className="App">
            <BrowserRouter>
                <Routes>
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/login" element={<LoginPage />} />

                    <Route element={<RootLayout />} >
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
                    </Route>

                    {/* Default fallback */}
                    <Route path="*" element={<Navigate to="/login" replace />} />
                </Routes>
            </BrowserRouter>
        </div>
    );
}

export default App;
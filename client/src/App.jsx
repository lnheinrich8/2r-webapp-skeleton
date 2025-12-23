import { useState } from 'react'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import { AuthProvider } from './features/auth/AuthContext';
import AppLayout from './features/shared/wrappers/AppLayout';

import Login from './features/login/Login';
import Register from './features/register/Register';
import Dashboard from './features/dashboard/Dashboard';

import './general.css';

const App = () => {

    return (
        <AuthProvider>
            <Router>
                <Routes>

                    <Route path="/" element={<Login />} />
                    <Route path="/register" element={<Register />} />

                    <Route element={<AppLayout />}>
                        <Route path="/dashboard" element={<Dashboard />} />
                    </Route>

                </Routes>
            </Router>
        </AuthProvider>
    );
}

export default App

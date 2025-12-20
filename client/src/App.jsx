import { useState } from 'react'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import Login from './features/login/Login';
import Register from './features/register/Register';
import Dashboard from './features/dashboard/Dashboard';

import './general.css';

const App = () => {

    return (
        <Router> {/* TODOO add AuthProvider as parent of Router */}
            <Routes>

                <Route path="/" element={<Login />} />
                <Route path="/register" element={<Register />} />

                {/* TODOO add AppLayout as parent of pages (besides login and register) */}
                <Route path="/dashboard" element={<Dashboard />} />

            </Routes>
        </Router>
    )
}

export default App

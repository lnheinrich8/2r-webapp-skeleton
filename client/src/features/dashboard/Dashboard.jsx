import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../auth/AuthContext';

// Components
import Loading from '../shared/components/Loading';
import Test from './components/Test';

// Styling
import './dashboard.css';

const Dashboard = () => {
    const { user, loading } = useAuth(); // user authorization


    if (loading) return <Loading />
    if (!user) return <Navigate to="/" />;

    return (
        <div className="dashboard-main">
            <Test />
        </div>
    )
}

export default Dashboard;

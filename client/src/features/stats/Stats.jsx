import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../auth/AuthContext';

// Components
import Loading from '../shared/components/Loading';

// Styling
import './stats.css';

const Stats = () => {
    const { user, loading } = useAuth(); // user authorization


    if (loading) return <Loading />
    if (!user) return <Navigate to="/" />

    return (
        <div className="stats-main">

        </div>
    );
}

export default Stats;

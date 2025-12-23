import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../../../auth/AuthContext';

// Styling
import './usertile.css';

const UserTile = () => {
    const { user } = useAuth(); // for user info

    return (
        <div className="usertile-main">
            <p className="usertile-name">Tester Testing</p>
        </div>
    )
}

export default UserTile;

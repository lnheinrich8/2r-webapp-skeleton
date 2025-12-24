import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../../../auth/AuthContext';

// Styling
import './usertile.css';

// Assets
import default_pfp from '../../../../assets/default_pfp.png';

const UserTile = () => {
    const { user, loading } = useAuth(); // for user info

    if (loading || !user) return null;

    return (
        <div className="usertile-main">
            <img className="usertile_pfp" src={default_pfp}/>

            <div className="usertile-name">
                <p className="usertile-firstname">{user.firstname}</p>
                <p className="usertile-lastname">{user.lastname}ss</p>
            </div>
        
        </div>
    )
}

export default UserTile;

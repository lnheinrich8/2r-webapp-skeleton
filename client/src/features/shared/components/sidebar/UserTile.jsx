import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../../../auth/AuthContext';

// Styling
import './usertile.css';

// Assets
import default_pfp from '../../../../assets/default_pfp.png';

const UserTile = () => {
    const { user } = useAuth(); // for user info

    return (
        <div className="usertile-main">
            <img className="usertile_pfp" src={default_pfp}/>
            <p className="usertile-name">Tester Testing</p>
        </div>
    )
}

export default UserTile;

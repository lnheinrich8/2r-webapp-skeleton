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

            <div className="usertile-name">
                <p className="usertile-firstname">Luke</p>
                <p className="usertile-lastname">Heinrich</p>
            </div>
        
        </div>
    )
}

export default UserTile;

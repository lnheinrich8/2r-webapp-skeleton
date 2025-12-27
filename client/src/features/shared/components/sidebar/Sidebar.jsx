import { Link, useLocation, useNavigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../../../auth/AuthContext';
import API_BASE_URL from '../../utils/api';

// Components
import UserTile from './UserTile';

// Styling
import './sidebar.css';

// Assets
import dashboard_icon from '../../../../assets/dashboard_icon.png'; // lmao this path
import stats_icon from '../../../../assets/stats_icon.png';

const Sidebar = ({ collapsed, setSidebarCollapsed, onOpenSettings }) => {
    const { setUser } = useAuth();

    const location = useLocation();
    const navigate = useNavigate();

    const handleLogout = async () => {
        try {
            await axios.get(`${API_BASE_URL}/auth/logout`, { withCredentials: true });
            setUser(null);
            navigate('/');
        } catch (error) {
            console.error('Logout failed:', error);
        }
    }

    return (
        <div className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
            <div className="top-sidebar-container">

                <button className="collapse-button" onClick={() => setSidebarCollapsed(prev => !prev)}>
                    <span className="collapse-icon">☰</span>
                </button>

                <UserTile onOpenSettings={onOpenSettings} />

            </div>

            <div className="sidebar-content">
                <div className="sidebar-links">
                    <Link to="/dashboard">
                        <button className={`sidebar-button ${location.pathname === '/dashboard' ? 'active' : ''}`}>
                            <img
                                src={dashboard_icon}
                                alt="Dashboard Icon"
                                draggable="false"
                                className={`sidebar-button-icon ${location.pathname === '/dashboard' ? 'active' : ''}`}
                            />
                            {!collapsed && <span className="sidebar-button-text">Dashboard</span>}
                        </button>
                    </Link>

                    <Link to="/stats">
                        <button className={`sidebar-button ${location.pathname === '/stats' ? 'active' : ''}`}>
                            <img
                                src={stats_icon}
                                alt="Stats Icon"
                                draggable="false"
                                className={`sidebar-button-icon ${location.pathname === '/stats' ? 'active' : ''}`}
                            />
                            {!collapsed && <span className="sidebar-button-text">Stats</span>}
                        </button>
                    </Link>
                </div>

                <div className="sidebar-footer">
                    <button className="logout-button" onClick={handleLogout}>
                        <span className="logout-icon">◄</span>
                        {!collapsed && <span className="logout-text">Logout</span>}
                    </button>
                </div>
            </div>

        </div>
    );
}

export default Sidebar;

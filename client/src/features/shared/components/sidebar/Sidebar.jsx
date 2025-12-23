import { Link, useLocation } from 'react-router-dom';

// Components
import UserTile from './UserTile';

// Styling
import './sidebar.css';

// Assets
import dashboard_icon from '../../../../assets/dashboard_icon.png'; // lmao this path
import stats_icon from '../../../../assets/stats_icon.png';

const Sidebar = ({ collapsed, setSidebarCollapsed }) => {
    const location = useLocation();

    return (
        <div className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
            <button className="collapse-button" onClick={() => setSidebarCollapsed(prev => !prev)}>
                <span className="collapse-icon">☰</span>
            </button>

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
    );
}

export default Sidebar;

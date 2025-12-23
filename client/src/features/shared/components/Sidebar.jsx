import { Link, useLocation } from 'react-router-dom';

// Styling
import './sidebar.css';

// Assets
import dashboard_icon from '../../../assets/dashboard_icon.png';

const Sidebar = ({ collapsed, setSidebarCollapsed }) => {

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

        </div>
    );
}

export default Sidebar;

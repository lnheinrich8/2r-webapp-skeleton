import { Link, useLocation } from 'react-router-dom';

// Styling
import './sidebar.css';

const Sidebar = ({ collapsed, setSidebarCollapsed }) => {

    return (
        <div className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
            <button className="collapse-button" onClick={() => setSidebarCollapsed(prev => !prev)}>
                <span className="collapse-icon">☰</span>
            </button>

            

        </div>
    );
}

export default Sidebar;

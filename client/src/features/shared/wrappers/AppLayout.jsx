import { Outlet } from 'react-router-dom';
import { useState } from 'react';
import Sidebar from '../components/sidebar/Sidebar';

const AppLayout = () => {
    const [sidebarCollapsed, setSidebarCollapsed] = useState(true);

    return (
        <div className="app-layout">
            <Sidebar collapsed={sidebarCollapsed} setSidebarCollapsed={setSidebarCollapsed} />
            <div className={`notsidebar-container ${sidebarCollapsed ? 'collapsed' : ''}`}>
                <Outlet />
            </div>
        </div>
    );
};

export default AppLayout;

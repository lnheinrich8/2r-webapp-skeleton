import { Outlet } from 'react-router-dom';
import { useState } from 'react';
import Sidebar from './components/Sidebar';

const AppLayout = () => {
    const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

    return (
        <div className="app-layout">
            <Sidebar collapsed={sidebarCollapsed} setSidebarCollapsed={setSidebarCollapsed} />
            <div className={`notsidebar-container ${sidebarCollapsed ? 'expanded' : ''}`}>
                <Outlet />
            </div>
        </div>
    );
};

export default AppLayout;

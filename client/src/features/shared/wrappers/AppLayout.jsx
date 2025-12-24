import { Outlet } from 'react-router-dom';
import { useState } from 'react';

// Components
import Sidebar from '../components/sidebar/Sidebar';
import SettingsModal from '../components/settings-modal/SettingsModal';

const AppLayout = () => {
    const [sidebarCollapsed, setSidebarCollapsed] = useState(true);
    const [settingsOpen, setSettingsOpen] = useState(false);

    return (
        <div className="app-layout">
            <SettingsModal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />

            <Sidebar
                collapsed={sidebarCollapsed}
                setSidebarCollapsed={setSidebarCollapsed}
                onOpenSettings={() => setSettingsOpen(true)}
            />
            <div className={`notsidebar-container ${sidebarCollapsed ? 'collapsed' : ''}`}>
                <Outlet />
            </div>
        </div>
    );
};

export default AppLayout;

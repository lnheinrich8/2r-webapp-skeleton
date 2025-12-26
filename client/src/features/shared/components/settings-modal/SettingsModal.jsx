import { useState } from 'react';

// Components
import ProfileContent from './components/ProfileContent';
import DataContent from './components/DataContent';

// Styling
import './settings-modal.css';

const sections = [
    { id: 'profile', label: 'Profile', component: ProfileContent },
    { id: 'data', label: 'Data', component: DataContent },
];

const SettingsModal = ({ isOpen, onClose }) => {
    if (!isOpen) return null;

    // Showing content based on active section clicked (default profile content)
    const [activeSection, setActiveSection] = useState('profile');
    const ActiveComponent = sections.find(section => section.id === activeSection)?.component ?? ProfileContent;

    const handleOverlayMouseDown = (event) => {
        if (event.target === event.currentTarget) {
            onClose();
        }
    };

    return (
        <div className="sm-overlay" onMouseDown={handleOverlayMouseDown}>
            <div className="sm-main" role="dialog" aria-modal="true">
                <button className="settings-modal-close" aria-label="Close settings" onClick={onClose}>
                    ×
                </button>
                <h2 className="sm-title">Settings</h2>

                <hr className="sm-horizontal-divider" />

                <div className="sm-section-content-container">
                    <div className="sm-sections-container">
                        {sections.map(section => (
                            <button
                                key={section.id}
                                type="button"
                                className={`sm-section-button ${activeSection === section.id ? 'active' : ''}`}
                                onClick={() => setActiveSection(section.id)}
                            >
                                {section.label}
                            </button>
                        ))}
                    </div>

                    <div className="sm-vertical-divider" />

                    <div className="sm-content-container">
                        <ActiveComponent />
                    </div>
                </div>
            </div>
        </div>
    );
}

export default SettingsModal;

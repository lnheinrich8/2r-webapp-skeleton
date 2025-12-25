// Styling
import './settings-modal.css';

const SettingsModal = ({ isOpen, onClose }) => {
    if (!isOpen) return null;

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

                    </div>

                    <vr className="sm-vertical-divider" />

                    <div className="sm-content-container">

                    </div>
                </div>
            </div>
        </div>
    );
}

export default SettingsModal;

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
        <div className="settings-modal-overlay" onMouseDown={handleOverlayMouseDown}>
            <div className="settings-modal-main" role="dialog" aria-modal="true">
                <button className="settings-modal-close" aria-label="Close settings" onClick={onClose}>
                    ×
                </button>
                <h2 className="settings-modal-title">Settings</h2>
                <p className="settings-modal-body">Settings content goes here.</p>
            </div>
        </div>
    );
}

export default SettingsModal;

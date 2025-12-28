import { useState } from 'react';
import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, loading } = useAuth();

    const [isEditing, setIsEditing] = useState(false);


    const handleEditToggle = () => {
        if (!isEditing) {
            setIsEditing(true);
            return;
        }

        // Exit edit mode
        setIsEditing(false);
    }




    if (loading || !user) return null;

    return (
        <div className="sm-content-main">

            <p className="sm-content-header">Edit</p>

            <div className="sm-content-row">
                <span className="sm-content-label">Edit profile</span>
                <button className="sm-content-button" onClick={handleEditToggle}>
                    {isEditing ? 'Save' : 'Edit'}
                </button>
            </div>

            <hr className="sm-content-horizontal-divider" />

            <p className="sm-content-header">Info</p>

            <div className="sm-content-row">
                <span className="sm-content-label">First Name</span>

                <span className="sm-content-value">
                    {isEditing ? (
                        <input
                            className="sm-content-edit-input"
                            type="firstname"
                            value={user.firstname}
                        />
                    ) : user.firstname}
                </span>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Last Name</span>

                <span className="sm-content-value">
                    {isEditing ? (
                        <input
                            className="sm-content-edit-input"
                            type="lastname"
                            value={user.lastname}
                        />
                    ) : user.lastname}
                </span>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Email</span>

                <span className="sm-content-value">
                    {isEditing ? (
                        <input
                            className="sm-content-edit-input"
                            type="email"
                            value={user.email}
                        />
                    ) : user.email}
                </span>
            </div>

            <hr className="sm-content-horizontal-divider" />

            <p className="sm-content-header">Security</p>

            <div className="sm-content-row">
                <span className="sm-content-label">Change password</span>
                <button className="sm-content-button-red">Change</button>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Delete account</span>
                <button className="sm-content-button-red">Delete</button>
            </div>

        </div>
    )
}

export default ProfileContent;

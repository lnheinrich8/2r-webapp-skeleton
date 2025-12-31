import { useState, useRef } from 'react';
import axios from 'axios';

import { useAuth } from '../../../../auth/AuthContext';
import API_BASE_URL from '../../../utils/api';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, setUser, loading } = useAuth();

    // Information updating stuff
    const [isEditing, setIsEditing] = useState(false);
    const firstRef = useRef(null);
    const lastRef = useRef(null);
    const emailRef = useRef(null);

    const handleEditToggle = async () => {
        if (!isEditing) { // if starting to edit
            setIsEditing(true);
            return;
        } // else: saving

        const inputFirst = firstRef.current.value;
        const inputLast = lastRef.current.value;
        const inputEmail = emailRef.current.value;

        // If everything is the same as before do nothing
        if (inputFirst == user.firstname && inputLast == user.lastname && inputEmail == user.email) {
            setIsEditing(false);
            return;
        }

        // If either first name or last name is different make API update request
        if (inputFirst != user.firstname || inputLast != user.lastname) {
            try {
                const res = await axios.patch(
                    `${API_BASE_URL}/user/update`,
                    { firstname: inputFirst, lastname: inputLast },
                    { withCredentials: true }
                );
                setUser(res.data);
            } catch (err) {
                console.error("Failed to update user info:", err);
            }
        }

        if (inputEmail != user.email) {
            try {
                const res = await axios.patch(
                    `${API_BASE_URL}/user/updatemail`,
                    { email: inputEmail },
                    { withCredentials: true }
                );
                setUser(res.data);
            } catch (err) {
                console.error("Failed to update user email:", err);
            }
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
                            defaultValue={user.firstname}
                            ref={firstRef}
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
                            defaultValue={user.lastname}
                            ref={lastRef}
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
                            defaultValue={user.email}
                            ref={emailRef}
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

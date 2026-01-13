import { useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

import { useAuth } from '../../../../auth/AuthContext';
import API_BASE_URL from '../../../utils/api';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, setUser, loading } = useAuth();
    const navigate = useNavigate(); // for changing password or deleting account

    // Information updating stuff
    const [isEditing, setIsEditing] = useState(false);
    const [isConfirmingDelete, setIsConfirmingDelete] = useState(false);
    const [isChangingPassword, setIsChangingPassword] = useState(false);
    const [currentPass, setCurrentPass] = useState('');
    const [newPass, setNewPass] = useState('');
    const firstRef = useRef(null);
    const lastRef = useRef(null);
    const emailRef = useRef(null);
    const deleteButtonRef = useRef(null);
    const editButtonRef = useRef(null);
    const currentPassRef = useRef(null);
    const newPassRef = useRef(null);
    const changePasswordButtonRef = useRef(null);


    // Profile editing event listener (to hide inputs)
    useEffect(() => {
        if (!isEditing) return;

        const handleDocumentMouseDown = (event) => {
            const clickedInput =
                firstRef.current?.contains(event.target) ||
                lastRef.current?.contains(event.target) ||
                emailRef.current?.contains(event.target);
            if (clickedInput || editButtonRef.current?.contains(event.target)) return;

            setIsEditing(false);
        };

        document.addEventListener('mousedown', handleDocumentMouseDown);
        return () => document.removeEventListener('mousedown', handleDocumentMouseDown);
    }, [isEditing]);

    // Change password event listener (to hide inputs)
    useEffect(() => {
        if (!isChangingPassword) return;

        const handleDocumentMouseDown = (event) => {
            const clickedInput =
                currentPassRef.current?.contains(event.target) ||
                newPassRef.current?.contains(event.target);
            if (clickedInput || changePasswordButtonRef.current?.contains(event.target)) return;

            setCurrentPass('');
            setNewPass('');
            setIsChangingPassword(false);
        };

        document.addEventListener('mousedown', handleDocumentMouseDown);
        return () => document.removeEventListener('mousedown', handleDocumentMouseDown);
    }, [isChangingPassword]);

    // Delete account event listener (to cancel)
    useEffect(() => {
        if (!isConfirmingDelete) return;

        const handleDocumentMouseDown = (event) => {
            if (deleteButtonRef.current && !deleteButtonRef.current.contains(event.target)) {
                setIsConfirmingDelete(false);
            }
        };

        document.addEventListener('mousedown', handleDocumentMouseDown);
        return () => document.removeEventListener('mousedown', handleDocumentMouseDown);
    }, [isConfirmingDelete]);

    const handleEditToggle = async () => {
        setIsConfirmingDelete(false);
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

                console.log(res.data); // TODO: show this notification in the UI
            } catch (err) {
                console.error("Failed to update user email:", err);
            }
        }

        // Exit edit mode
        setIsEditing(false);
    }

    const handleChangePassword = async () => {
        if (!isChangingPassword) {
            setIsChangingPassword(true);
            return;
        }

        if (!currentPass || !newPass) {
            console.error("Current password and new password cannot be empty"); // TODO: show this error in the UI
            return;
        }

        if (currentPass == newPass) {
            console.error("New password must be different from the current password"); // TODO: show this an error in the UI
            return;
        }

        try {
            await axios.patch(
                `${API_BASE_URL}/auth/newpass`,
                { current_pass: currentPass, new_pass: newPass },
                { withCredentials: true }
            )
            setCurrentPass('');
            setNewPass('');
            setIsChangingPassword(false);
            // TODO: show successful password change in the UI
        } catch (err) {
            console.error("Could not change password"); // TODO: show this an error in the UI
            console.error(err); // Keep this in the console
        }
    }

    const handleDelete = async () => {
        setIsEditing(false);
        if (!isConfirmingDelete) {
            setIsConfirmingDelete(true);
            return;
        }

        try {
            await axios.delete(
                `${API_BASE_URL}/user/delete`,
                { withCredentials: true }
            )
            setUser(null);
            navigate('/');
        } catch (err) {
            console.error('Deleting user failed:', err);
        } finally {
            setIsConfirmingDelete(false);
        }
    }


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">

            <p className="sm-content-header">Edit</p>

            <div className="sm-content-row">
                <span className="sm-content-label">Edit profile</span>
                <button
                    className="sm-content-button"
                    onClick={handleEditToggle}
                    ref={editButtonRef}
                >
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

            <div>
                <div className="sm-content-row">
                    <span className="sm-content-label">Change password</span>
                    <button
                        className="sm-content-button-red"
                        onClick={handleChangePassword}
                        ref={changePasswordButtonRef}
                    >
                        {isChangingPassword ? 'Confirm' : 'Change'}
                    </button>
                </div>

                <div className={`sm-content-password-fields ${isChangingPassword ? 'is-open' : ''}`}>
                    <div className="sm-content-password-inputs">
                        <input
                            className="sm-content-edit-input"
                            type="password"
                            placeholder="Current password"
                            value={currentPass}
                            onChange={(event) => setCurrentPass(event.target.value)}
                            ref={currentPassRef}
                        />
                        <input
                            className="sm-content-edit-input"
                            type="password"
                            placeholder="New password"
                            value={newPass}
                            onChange={(event) => setNewPass(event.target.value)}
                            ref={newPassRef}
                        />
                    </div>
                </div>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Delete account</span>
                <button
                    ref={deleteButtonRef}
                    className="sm-content-button-red"
                    onClick={handleDelete}
                >
                    {isConfirmingDelete ? 'Confirm' : 'Delete'}
                </button>
            </div>

        </div>
    )
}

export default ProfileContent;

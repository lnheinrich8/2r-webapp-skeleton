import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">

            <h className="sm-content-header">Edit</h>

            <div className="sm-content-row">
                <span className="sm-content-label">Edit profile</span>
                <button className="sm-content-button">Edit</button>
            </div>

            <hr className="sm-content-horizontal-divider" />

            <h className="sm-content-header">Info</h>

            <div className="sm-content-row">
                <span className="sm-content-label">First Name</span>
                <span className="sm-content-value">{user.firstname}</span>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Last Name</span>
                <span className="sm-content-value">{user.lastname}</span>
            </div>

            <div className="sm-content-row">
                <span className="sm-content-label">Email</span>
                <span className="sm-content-value">{user.email}</span>
            </div>

            <hr className="sm-content-horizontal-divider" />

            <h className="sm-content-header">Security</h>

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

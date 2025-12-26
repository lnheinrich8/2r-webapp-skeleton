import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">
            
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
        
        </div>
    )
}

export default ProfileContent;

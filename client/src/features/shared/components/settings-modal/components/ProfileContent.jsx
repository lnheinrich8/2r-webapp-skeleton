import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const ProfileContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">
            <p>{user.firstname}</p>
            <p>{user.lastname}</p>
            <p>{user.email}</p>
        </div>
    )
}

export default ProfileContent;
